pub use crate::{
    context::Context,
    error::Error,
    types::*,
    websocket::{DiscordMessage, Websocket},
    client::Client,
};

pub use anyhow::Result;
pub use http::Method;
pub use std::sync::Arc;
pub use serde_json::{self, json, Value};
pub use tokio::sync::{Mutex, MutexGuard};
pub use prost::Message as ProstMessage;

pub type Ctx = Arc<Mutex<Context>>;
pub type Ws = Arc<Mutex<Websocket>>;
pub type Model<M> = Arc<Mutex<M>>;
