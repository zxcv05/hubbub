use serde::{Deserialize, Serialize};

use super::Snowflake;


/**
 * This is stupid
 * So, fields of type Option<()> "represent booleans":
 * if it exists, its true. otherwise false
 * use .is_some()
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct RoleTag {
    bot_id: Option<Snowflake>,
    integration_id: Option<Snowflake>,
    subscription_listing_id: Option<Snowflake>,

    premium_subscriber: Option<()>,
    available_for_purchase: Option<()>,
    guild_connections: Option<()>,
}

#[derive(Debug)]
#[repr(u8)]
pub enum RoleFlags {
    InPrompt = 1 << 0,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    id: Snowflake,
    name: String,
    color: u64, // 0x--RRGGBB i assume
    hoist: bool,

    position: u64,
    permissions: String, // String because bigint

    managed: bool,
    mentionable: bool,

    tags: Option<RoleTag>,

    flags: u8,

    icon: Option<String>, // icon hash
    #[serde(rename = "unicode_emoji")]
    emoji: Option<String>,

}

