use serde::{Deserialize, Serialize};

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


