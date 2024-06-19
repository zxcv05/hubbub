use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::Snowflake;

/**
 * This is stupid
 * So, fields of type Option<()> "represent booleans":
 * if it exists, it's true. otherwise false
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: Snowflake,
    pub tier_name: String,
    pub total_months_subscribed: u64,
    pub is_renewal: bool,
}

#[derive(Debug)]
#[repr(u8)]
pub enum RoleFlags {
    InPrompt = 1 << 0,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Role {
    pub id: Snowflake,
    pub name: String,
    pub color: u64, // 0x--RRGGBB i assume
    pub hoist: bool,

    pub position: u64,
    pub permissions: String, // String because bigint

    pub managed: bool,
    pub mentionable: bool,

    pub tags: Option<RoleTag>,

    pub flags: u8,

    pub icon: Option<String>, // icon hash
    #[serde(rename = "unicode_emoji")]
    pub emoji: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleBuilder {
    value: Value,
}

impl RoleBuilder {
    pub fn new(name: String, permissions: String) -> Self {
        Self {
            value: json!({
                "name": name,
                "permissions": permissions
            }),
        }
    }

    pub fn set_color(&mut self, color: u64) {
        self.value["color"] = json!(color);
    }

    pub fn set_hoist(&mut self, hoist: bool) {
        self.value["hoist"] = json!(hoist);
    }

    // icon is data:image/jpeg;base64,..
    pub fn set_icon(&mut self, icon: String) {
        self.value["icon"] = json!(icon);
    }

    pub fn set_emoji(&mut self, emoji: String) {
        self.value["unicode_emoji"] = json!(emoji);
    }

    pub fn set_mentionable(&mut self, mentionable: bool) {
        self.value["mentionable"] = json!(mentionable);
    }

    pub fn build(self) -> Value {
        self.value
    }
}
