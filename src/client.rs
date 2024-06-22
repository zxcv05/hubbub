use std::future::Future;
use crate::prelude::{Ctx, Model, Ws};
use anyhow::Result;
use crate::context::{Context, ResumeInfo};
use crate::error::Error;
use std::{cmp::max, collections::VecDeque, sync::Arc, time::Duration};
use tokio::sync::Mutex;
use crate::types::gateway::Ready;
use crate::websocket::{DiscordMessage, Websocket};
pub type Handler<F, M> = dyn Fn(Ctx, Ws, Model<M>, DiscordMessage) -> F;

pub struct Client<F, Model>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    ws: Arc<Mutex<Websocket>>,
    ctx: Arc<Mutex<Context>>,
    handler: Box<Handler<F, Model>>,
    model: Arc<Mutex<Model>>,
}

impl<F, Model> Client<F, Model>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    pub async fn new(model: Model, handler: Box<Handler<F, Model>>) -> Result<Self> {
        Ok(Self {
            ws: Arc::new(Mutex::from(Websocket::new().await?)),
            ctx: Arc::new(Mutex::from(Context::default())),
            model: Arc::new(Mutex::from(model)),
            handler,
        })
    }

    pub async fn token(&mut self, token: String) -> Result<()> {
        self.ws.lock().await.token(token.clone());

        let mut ctx = self.ctx.lock().await;
        ctx.set_auth(token);

        match ctx.request(http::Method::GET, "/v9/users/@me", None).await {
            Ok(r) => {
                // If return value was an error
                if r.body.get("code").is_some() && r.body.get("message").is_some() {
                    Err(Error::InvalidToken(
                        r.body.get("message").unwrap().as_str().unwrap().to_string(),
                    )
                        .into())
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(e.context("Failed validating token")),
        }
    }

    pub async fn login(&mut self) -> Result<()> {
        self.ws.lock().await.login().await?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let ws = self.ws.lock().await;

        let ws_ref = self.ws.clone();
        let hb = ws.heartbeat;
        tokio::task::spawn(async move {
            let mut i = async_timer::Interval::platform_new(Duration::from_millis(hb));

            loop {
                i.as_mut().await;

                let mut lock = ws_ref.lock().await;
                lock.heartbeat().await.expect("Couldn't heartbeat");
            }
        });

        drop(ws);

        let mut dispatch_queue: VecDeque<DiscordMessage> = VecDeque::with_capacity(8);
        loop {
            let mut ws = match self.ws.try_lock() {
                Ok(ws) => ws,
                Err(_) => {
                    tokio::time::sleep(Duration::from_millis(2)).await;
                    continue;
                }
            };

            let mut seq = ws.sequence;
            while let Some(msg) = ws.try_read().await? {
                match msg.op {
                    0 => {
                        log::trace!(
                            "Got dispatch event #{}: {}",
                            msg.seq.as_ref().unwrap(),
                            msg.event.as_ref().unwrap()
                        );
                        seq = max(seq, msg.seq);
                        dispatch_queue.push_back(msg)
                    }
                    1 => {
                        log::trace!("Gateway asked for heartbeat");
                        ws.heartbeat().await?;
                    }
                    7 => {
                        log::trace!("Gateway asked for reconnect");
                        let ctx = self.ctx.lock().await;
                        let seq = ws.sequence;
                        *ws = Websocket::new_with(
                            &(ctx.resume_info.as_ref().unwrap().url),
                            seq.unwrap(),
                        )
                            .await?;
                        ws.token(ctx.auth.as_ref().unwrap().clone());
                        ws.resume(ctx.resume_info.as_ref().unwrap()).await?;
                    }
                    11 => {
                        log::trace!("Gateway acknowledged heartbeat");
                    }
                    _ => (),
                }
            }
            ws.sequence = seq;
            drop(ws);

            while !dispatch_queue.is_empty() {
                if let Some(msg) = dispatch_queue.pop_front() {
                    let event = msg.event.as_ref().unwrap();

                    if event.as_str() == "RESUMED" {
                        log::debug!("Resumed");
                    }

                    if event.as_str() == "READY" {
                        // this clone is DISGUSTING
                        let mut ready: Ready =
                            serde_json::from_value(msg.data.clone()).expect("Couldn't parse READY");

                        let mut ctx = self.ctx.lock().await;
                        ctx.user = Some(ready.user);
                        ctx.resume_info = Some(ResumeInfo {
                            url: ready.resume_gateway_url,
                            id: ready.session_id,
                        });

                        ctx.cache.guilds.append(&mut ready.cached_guilds);
                        ctx.cache.users.append(&mut ready.cached_users);

                        log::trace!("Context after READY: {ctx:?}");
                    }

                    (self.handler)(self.ctx.clone(), self.ws.clone(), self.model.clone(), msg)
                        .await;
                }
            }

            dispatch_queue.clear();
            tokio::time::sleep(Duration::from_millis(2)).await;
        }
    }
}
