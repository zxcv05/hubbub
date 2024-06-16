use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::Snowflake;

#[derive(Deserialize, Serialize, Debug)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserClan {
    #[serde(rename = "identity_enabled")]
    pub shown: bool,
    pub badge: Option<String>,
    pub identity_guild_id: Option<String>,
    pub tag: Option<String>,
}


#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum RelationshipType {
    User = 1,
    Bot = 2,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Relationship {
    pub id: String,
    pub nickname: Option<String>,
    pub since: Option<String>,
    #[serde(rename = "type")]
    pub user_type: RelationshipType,
}


#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
    NitroBasic = 3
}

#[repr(u32)]
pub enum UserFlags {
    Staff               = 1 << 0,
    Partner             = 1 << 1,
    Hypesquad           = 1 << 2,
    BugHunterLvl1       = 1 << 3,
    HypesquadBravery    = 1 << 6,
    HypesquadBrilliance = 1 << 7,
    HypesquadBalance    = 1 << 8,
    EarlyNitroSupporter = 1 << 9,
    TeamPseudoUser      = 1 << 10,
    BugHunterLvl2       = 1 << 14,
    VerifiedBot         = 1 << 16,
    VerifiedDev         = 1 << 17,
    CertifiedMod        = 1 << 18,
    BotHTTPInteractions = 1 << 19, // bot uses only http interactions
    ActiveDev           = 1 << 22,
}

fn default_bot_value() -> bool { false }
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: Snowflake,
    pub username: String,

    pub global_name: Option<String>, // bots: application name

    pub avatar: Option<String>, // avatar hash
    pub avatar_decoration_data: Option<AvatarDecorationData>,

    pub banner: Option<String>, // banner hash
    pub accent_color: Option<u32>, // hex color

    pub locale: Option<String>,

    #[serde(rename = "bot", default = "default_bot_value")]
    pub is_bot: bool,
    #[serde(rename = "system", default = "default_bot_value")]
    pub is_system: bool,

    // These two are oath2 "email" scope only
    #[serde(rename = "verified")]
    pub is_verified: Option<bool>,
    pub email: Option<String>,

    pub mfa_enabled: Option<bool>,

    pub public_flags: Option<u32>,

    pub premium_type: Option<PremiumType>, 
}


#[derive(Deserialize, Serialize, Debug)]
pub struct BotUser {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub pronouns: String,
    pub bio: String,
    
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    
    pub banner: Option<String>,
    pub banner_color: String,
    pub accent_color: u64,
    
    pub clan: Option<UserClan>,
    
    #[serde(rename = "desktop")]
    pub on_desktop: bool,
    #[serde(rename = "mobile")]
    pub on_mobile: bool,
    #[serde(rename = "verified")]
    pub is_verified: bool,
    #[serde(rename = "premium")]
    pub has_premium: bool,
    
    pub email: Option<String>,
    pub phone: Option<String>,
    
    pub mfa_enabled: bool,
    pub nsfw_allowed: bool,
    
    pub premium_type: u32,
    
    pub purchased_flags: u32,
    pub public_flags: u64,
    pub flags: u64,
}

