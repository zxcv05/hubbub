use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use crate::types::channel::Channel;
use crate::types::message::{Attachment, Message};
use crate::types::role::Role;

use super::{user::User, Snowflake};


#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    pub id: Option<Snowflake>,
    pub name: Option<String>, // null in reaction emojis objects
    pub roles: Vec<String>, // role ids (i think?)
    
    #[serde(rename = "user")]
    pub creator: Option<User>,

    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resolved {
    pub users: Option<HashMap<Snowflake, User>>,
    pub members: Option<HashMap<Snowflake, User>>, // TODO guild member
    pub roles: Option<HashMap<Snowflake, Role>>,
    pub channels: Option<HashMap<Snowflake, Channel>>,
    pub messages: Option<HashMap<Snowflake, Message>>,
    pub attachments: Option<HashMap<Snowflake, Attachment>>,
}

