use serde::{Deserialize, Serialize};

use super::activity::Activity;


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StatusType {
    Online,
    Dnd,
    Idle,
    Invisible,
    Offline,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GatewayPresence {
    pub since: Option<u128>, // in ms
    pub activities: Vec<Activity>,
    pub status: StatusType,
    pub afk: bool,
}

