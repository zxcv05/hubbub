
use crate::{context::ResumeInfo, error::Error};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JSON};
use tokio::sync::Mutex;
use std::{collections::VecDeque, sync::Arc, thread::sleep, time::Duration};
use reqwest_websocket::{websocket, Message, WebSocket as WS};


static DISCORD_WS_URI: &'static str = "wss://gateway.discord.gg/?encoding=json&v=9";

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordMessage {
    pub op: u8,
    
    #[serde(rename = "d")]
    pub data: JSON,
    
    #[serde(rename = "s", skip_serializing)]
    pub seq: Option<u64>,

    #[serde(rename = "t", skip_serializing)]
    pub event: Option<String>
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
            rx: VecDeque::with_capacity(128),
            tx: VecDeque::with_capacity(128),
        }
    }

    pub async fn start(self) -> (Arc<Mutex<VecDeque<Message>>>, Arc<Mutex<VecDeque<Message>>>) {
        let _txq = Arc::new(Mutex::new(self.tx));
        let _rxq = Arc::new(Mutex::new(self.rx));
        let (mut tx, mut rx) = self.s.into_stream().split();

        let rxq = _rxq.clone();
        tokio::task::spawn(async move {
            log::debug!("Starting websocket read loop");

            loop {
                let resp = match rx.try_next().await {
                    Ok(v) => v,
                    Err(e) => {
                        let mut rxq = rxq.lock().await;
                        rxq.close().await.expect("Couldn't close sink");
                        panic!("{e:?}");
                    },
                };

                if let Some(msg) = resp {
                    log::debug!("<<");
                    log::trace!("{msg:?}");

                    let mut rxq = rxq.lock().await;
                    rxq.push_back(msg);
                }

                sleep(Duration::from_millis(5));
            }
        });

        let txq = _txq.clone();
        tokio::task::spawn(async move {
            log::debug!("Starting websocket write loop");

            loop {
                let mut txq = match txq.try_lock() {
                    Ok(v) => v,
                    Err(_) => {
                        sleep(Duration::from_millis(5));
                        continue;
                    },
                };
    
                while let Some(msg) = txq.pop_front() {
                    log::debug!(">>");
                    log::trace!("{msg:?}");
                    tx.feed(msg).await.expect("Couldn't feed tx");
                }
                
                if let Err(e) = tx.flush().await {
                    txq.close().await.expect("Couldn't close stream");
                    panic!("{e:?}");
                }

                drop(txq);

                sleep(Duration::from_millis(5));
            }
        });

        (_txq, _rxq)
    }
}


pub struct Websocket {
    // tx, rx
    pub q: (Arc<Mutex<VecDeque<Message>>>, Arc<Mutex<VecDeque<Message>>>),
    
    pub ready: bool,
    
    pub heartbeat: u64,
    pub sequence: Option<u64>,
    
    token: Option<String>,
}

impl Websocket {
    pub async fn new() -> Result<Self> {
        let (tx, rx) = StreamCtrl::new(websocket(DISCORD_WS_URI).await?).start().await;

        Ok(Self {
            ready: false,
            token: None,
            heartbeat: 0,
            sequence: None,
            q: (tx, rx),
        })
    }

    pub async fn new_with(resume_gateway: &String, sequence: u64) -> Result<Self> {
        let (tx, rx) = StreamCtrl::new(websocket(format!("{}?encoding=json&v=9", resume_gateway)).await?).start().await;

        Ok(Self {
            ready: false,
            token: None,
            heartbeat: 0,
            sequence: Some(sequence),
            q: (tx, rx)
        })
    }

    pub fn token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub async fn send(&mut self, msg: DiscordMessage) -> Result<()> {
        let msg = Self::serialize_message(msg)?;

        let mut lock = self.q.0.lock().await;
        lock.push_back(msg);
        drop(lock);

        Ok(())
    }

    pub async fn read(&mut self) -> Result<DiscordMessage> {
        loop {
            let mut lock = self.q.1.lock().await;

            if lock.is_empty() {
                drop(lock);
                sleep(Duration::from_millis(5));
                continue;
            }

            break Ok(Self::parse_message(lock.pop_front().unwrap())?)
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

    pub fn resume_packet(&self, info: &ResumeInfo) -> DiscordMessage {
        DiscordMessage {
            op: 6,
            data: json!({
                "token": self.token,
                "session_id": info.id,
                "seq": self.sequence
            }),

            seq: None,
            event: None,
        }        
    }

    pub fn identify_packet(&self) -> DiscordMessage {
        DiscordMessage {
            op: 2,
            data: json!({
                "token": self.token,
                "capabilities": 30717,
                "properties": {
                    "os": "Windows",
                    "browser": "Firefox",
                    "device": "",
                    "system_locale": "en-US",
                    "browser_user_agent": "Mozilla/5.0 (Windows NT 10.0; rv:126.0) Gecko/20100101 Firefox/126.0",
                    "browser_version": "126.0",
                    "os_version": "",
                    "referrer": "",
                    "referring_domain": "",
                    "referrer_current": "",
                    "referring_domain_current": "",
                    "release_channel": "stable",
                    "client_build_number": 301409, // TODO: reliable way to get up-to-date build number
                    "client_event_source": JSON::Null,
                    "design_id": 0
                },
                "presence": {
                    "status": "unknown",
                    "since": 0,
                    "activities": [],
                    "afk": false
                },
                "compress": false,
                "client_state": {
                    "guild_versions": {},
                }
            }),

            seq: None,
            event: None,
        }
    }


    pub async fn initiate(&mut self) -> Result<()> {
        if let None = self.token {
            return Err(Error::NoTokenGiven.into());
        }
    
        log::debug!("Initating connection");
        
        let hello = self.read().await?;
        self.heartbeat = hello.data["heartbeat_interval"].as_u64().expect("Invalid heartbeat interval");
    
        self.heartbeat().await?;
        let _ = self.read().await?;

        Ok(())
    }

    pub async fn login(&mut self) -> Result<()> {
        self.initiate().await?;
        
        log::debug!("Sending identify packet");
        self.send(self.identify_packet()).await?;
        
        Ok(())
    }

    pub async fn resume(&mut self, info: &ResumeInfo) -> Result<()> {
        self.initiate().await?;
        
        log::debug!("Sending resume packet");
        self.send(self.resume_packet(info)).await?;

        Ok(())
    }

    pub async fn heartbeat(&mut self) -> Result<()> {
        log::debug!("Sending heartbeat");
        self.send(DiscordMessage {
            op: 1,
            data: match self.sequence {
                None => JSON::Null,
                Some(i) => JSON::Number(i.into()),
            },

            seq: None,
            event: None,
        }).await.expect("Couldn't send heartbeat");

        Ok(())
    }
}


