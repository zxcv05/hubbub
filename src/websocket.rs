use crate::{context::ResumeInfo, prelude::{Arc, Ctx, Mutex}};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest_websocket::{websocket, Message, WebSocket as WS};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JSON};
use std::{collections::VecDeque, time::Duration};

static DISCORD_WS_URI: &str = "wss://gateway.discord.gg/?encoding=json&v=9";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DiscordMessage {
    pub op: u8,

    #[serde(rename = "d")]
    pub data: JSON,

    #[serde(rename = "s", skip_serializing)]
    pub seq: Option<u64>,

    #[serde(rename = "t", skip_serializing)]
    pub event: Option<String>,
}

impl DiscordMessage {
    pub fn new_heartbeat(seq: Option<u64>) -> Self {
        Self {
            op: 1,
            data: match seq {
                None => JSON::Null,
                Some(s) => JSON::Number(s.into()),
            },
            seq: None,
            event: None,
        }
    }

    pub fn new_identify(token: &str) -> Self {
        // Yeah the formatting is strange but it's to provide easy access
        // to "token" and "client_build_number" without searching or scrolling
        Self {
            op: 2,
            data: json!({
                "token": token, "capabilities": 30717, "properties": { "os": "Windows", "browser": "Firefox", "device": "", "system_locale": "en-US", "browser_user_agent": "Mozilla/5.0 (Windows NT 10.0; rv:126.0) Gecko/20100101 Firefox/126.0", "browser_version": "126.0", "os_version": "", "referrer": "", "referring_domain": "", "referrer_current": "", "referring_domain_current": "", "release_channel": "stable",
                "client_build_number": 327180, "client_event_source": JSON::Null, "design_id": 0 }, "presence": { "status": "unknown", "since": 0, "activities": [], "afk": false }, "compress": false, "client_state": { "guild_versions": {} }
            }),
            seq: None,
            event: None,
        }
    }

    pub fn new_resume(token: &str, seq: u64, info: &ResumeInfo) -> Self {
        Self {
            op: 6,
            data: json!({
                "token": token,
                "session_id": info.id,
                "seq": seq,
            }),
            seq: None,
            event: None,
        }
    }
}

pub struct StreamCtrl {
    s: WS,
    rx: VecDeque<Message>,
    tx: VecDeque<Message>,
}

impl StreamCtrl {
    pub fn new(s: WS) -> Self {
        Self {
            s,
            rx: VecDeque::with_capacity(64),
            tx: VecDeque::with_capacity(64),
        }
    }

    pub async fn start(self) -> (Arc<Mutex<VecDeque<Message>>>, Arc<Mutex<VecDeque<Message>>>) {
        let _txq = Arc::new(Mutex::new(self.tx));
        let _rxq = Arc::new(Mutex::new(self.rx));
        let (mut tx, mut rx) = self.s.into_stream().split();

        let rxq = _rxq.clone();
        tokio::task::spawn(async move {
            log::trace!("Starting websocket read loop");

            loop {
                let msg = match rx.next().await {
                    Some(r) => match r {
                        Err(e) => {
                            let mut rxq = rxq.lock().await;
                            rxq.close().await.expect("Couldn't close sink");
                            rxq.push_back(Message::Text(String::from(r#"{"op":255}"#)));
                            log::error!("rxq: {e}");
                            return;
                        }
                        Ok(m) => m,
                    },
                    None => {
                        let mut rxq = rxq.lock().await;
                        rxq.close().await.expect("Couldn't close sink");
                        log::trace!("rxq closed because end-of-stream reached");
                        return;
                    }
                };

                log::trace!("<<\n{msg:?}");

                let mut rxq = rxq.lock().await;
                rxq.push_back(msg);
            }
        });

        let txq = _txq.clone();
        tokio::task::spawn(async move {
            log::trace!("Starting websocket write loop");

            loop {
                tokio::time::sleep(Duration::from_millis(5)).await;
                
                let mut txq = txq.lock().await;
                if txq.is_empty() { continue }

                while let Some(msg) = txq.pop_front() {
                    log::trace!(">>\n{msg:?}");
                    tx.feed(msg).await.expect("Couldn't feed tx");
                }

                if let Err(e) = tx.flush().await {
                    txq.close().await.expect("Couldn't close stream");
                    log::error!("{e:?}");
                    return;
                }

                drop(txq);
            }
        });

        (_txq, _rxq)
    }
}

type Queue = Arc<Mutex<VecDeque<Message>>>;

pub struct Websocket {
    // tx, rx
    pub q: (Queue, Queue),

    pub ready: bool,

    pub heartbeat: u64,
    pub sequence: Option<u64>,
}

impl Websocket {
    async fn connect(url: String) -> Result<WS> {
        Ok(loop {
            log::debug!("Trying websocket connection");
            let attempt = websocket(url.clone()).await;

            match attempt {
                Ok(v) => { break v },
                Err(e) => {
                    log::error!("Got error while trying to connect: {e}");
                    log::info!("Retrying in 5s");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        })
    }

    pub async fn new() -> Result<Self> {
        let ws = Self::connect(String::from(DISCORD_WS_URI)).await?;

        let (tx, rx) = StreamCtrl::new(ws)
            .start()
            .await;

        Ok(Self {
            ready: false,
            heartbeat: 0,
            sequence: None,
            q: (tx, rx),
        })
    }

    pub async fn new_with(resume_gateway: &String, sequence: u64) -> Result<Self> {
        let ws = Self::connect(format!("{}?encoding=json&v=9", resume_gateway)).await?;

        let (tx, rx) =
            StreamCtrl::new(ws)
                .start()
                .await;

        Ok(Self {
            ready: false,
            heartbeat: 0,
            sequence: Some(sequence),
            q: (tx, rx),
        })
    }

    pub async fn send(&mut self, msg: DiscordMessage) -> Result<()> {
        let msg = Self::serialize_message(msg)?;

        let mut lock = self.q.0.lock().await;
        lock.push_back(msg);

        Ok(())
    }

    pub async fn read(&mut self) -> Result<DiscordMessage> {
        loop {
            let mut lock = self.q.1.lock().await;

            if lock.is_empty() {
                drop(lock);
                tokio::time::sleep(Duration::from_millis(5)).await;
                continue;
            }

            break Ok(Self::parse_message(lock.pop_front().unwrap())?);
        }
    }

    pub async fn try_read(&mut self) -> Result<Option<DiscordMessage>> {
        let mut lock = self.q.1.lock().await;

        Ok(match lock.pop_front() {
            Some(v) => Some(Self::parse_message(v)?),
            None => None,
        })
    }

    pub fn parse_message(msg: Message) -> Result<DiscordMessage> {
        Ok(match msg {
            Message::Text(t) => serde_json::from_str(t.as_str())?,
            Message::Binary(b) => serde_json::from_slice(b.as_slice())?,
        })
    }

    pub fn serialize_message(msg: DiscordMessage) -> Result<Message> {
        Ok(Message::Text(serde_json::to_string(&msg)?))
    }

    async fn read_hello(&mut self) -> Result<()> {
        let hello = self.read().await?;
        self.heartbeat = hello.data["heartbeat_interval"]
            .as_u64()
            .expect("Invalid heartbeat interval");

        self.send(DiscordMessage::new_heartbeat(None)).await?;
        let _ = self.read().await?;

        Ok(())
    }

    pub async fn login(&mut self, token: &str) -> Result<()> {
        self.read_hello().await?;

        log::debug!("Sending identify packet");
        self.send(DiscordMessage::new_identify(token)).await?;

        Ok(())
    }

    pub async fn resume(&mut self, ctx: Ctx) -> Result<()> {
        self.read_hello().await?;

        log::debug!("Sending resume packet");

        let ctx = ctx.lock().await;
        let info = ctx.resume_info.as_ref().unwrap();
        self.send(DiscordMessage::new_resume(ctx.auth.as_ref().unwrap(), self.sequence.unwrap(), info)).await?;

        Ok(())
    }
}
