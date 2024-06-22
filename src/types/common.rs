use crate::types::channel::Channel;
use crate::types::guild::GuildMember;
use crate::types::message::{Attachment, Message};
use crate::types::role::Role;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{user::User, Snowflake};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>, // null in reaction emojis objects
    pub roles: Option<Vec<Snowflake>>,

    #[serde(rename = "user")]
    pub creator: Option<User>,

    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Resolved {
    pub users: Option<HashMap<Snowflake, User>>,
    pub members: Option<HashMap<Snowflake, GuildMember>>,
    pub roles: Option<HashMap<Snowflake, Role>>,
    pub channels: Option<HashMap<Snowflake, Channel>>,
    pub messages: Option<HashMap<Snowflake, Message>>,
    pub attachments: Option<HashMap<Snowflake, Attachment>>,
}
