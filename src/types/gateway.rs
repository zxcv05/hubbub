use serde::{Deserialize, Serialize};

use super::{
    channel::Channel,
    guild::CachedGuild,
    user::{BotUser, Relationship, User},
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Ready {
    pub private_channels: Vec<Channel>,

    #[serde(rename = "relationships")]
    pub friends: Vec<Relationship>,

    pub resume_gateway_url: String,
    pub session_id: String,
    pub session_type: String,

    pub user: BotUser,

    #[serde(rename = "users")]
    pub cached_users: Vec<User>,

    #[serde(rename = "guilds")]
    pub cached_guilds: Vec<CachedGuild>,
}
