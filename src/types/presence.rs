use serde::{Deserialize, Serialize};

use super::activity::Activity;

#[derive(Deserialize, Serialize, Debug)]
pub struct Presence {
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: bool,
}

