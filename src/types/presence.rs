use std::fmt::Display;
use serde::{Deserialize, Serialize};

use super::activity::Activity;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StatusType {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}

impl Display for StatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StatusType::Online => "online".to_string(),
            StatusType::Dnd => "dnd".to_string(),
            StatusType::Idle => "idle".to_string(),
            StatusType::Invisible => "invisible".to_string(),
            StatusType::Offline => "offline".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GatewayPresence {
    pub since: Option<u128>, // in ms
    pub activities: Vec<Activity>,
    pub status: StatusType,
    pub afk: bool,
}
