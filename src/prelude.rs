pub use crate::{
    context::Context,
    error::Error,
    types::*,
    websocket::{DiscordMessage, Websocket},
    Client,
};

pub use anyhow::Result;
pub use http::Method;
pub use serde_json::{self, json, Value};
pub use std::sync::Arc;
pub use tokio::sync::{Mutex, MutexGuard};

pub type Ctx = Arc<Mutex<Context>>;
pub type Ws = Arc<Mutex<Websocket>>;
pub type Model<M> = Arc<Mutex<M>>;