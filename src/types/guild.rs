use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{channel::{Channel, welcome_screen::WelcomeScreen}, common::Emoji, role::Role, sticker::Sticker, Snowflake};


#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum MFALevel {
    None = 0, // dont
    Elevated = 1, // need 2FA for moderation
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum VerificationLevel {
    None = 0,     // unrestricted
    Low = 1,      // verified email
    Medium = 2,   // registered for > 5min
    High = 3,     // member of server > 10min
    VeryHigh = 4, // verified phone #
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum MessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
    Disabled = 0, // no members get scanned
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

#[derive(Debug)]
#[repr(u8)]
pub enum SystemChannelFlags {
    SuppressJoinNotifications       = 1 << 0,
    SuppressPremiumSubscriptions    = 1 << 1,
    SuppressGuildReminders          = 1 << 2,
    SuppressJoinReplies             = 1 << 3,
    SuppressRoleSubPurchases        = 1 << 4,
    SuppressRoleSubPurchaseReplies  = 1 << 5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    pub id: Snowflake,
    pub name: String,
    
    pub owner_id: Snowflake,
    
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    
    #[serde(rename = "vanity_url_code")]
    pub vanity_invite: Option<String>,

    pub description: Option<String>,
    pub banner: Option<String>, // banner hash

    #[serde(rename = "discovery_splash")]
    pub splash_discovery: Option<String>,
    pub splash: Option<String>,

    #[serde(rename = "preferred_locale")]
    pub locale: String,

    pub mfa_level: MFALevel,
    pub nsfw_level: NSFWLevel,
    pub verification_level: VerificationLevel,
    #[serde(rename = "default_message_notifications")]
    pub default_notification_level: MessageNotificationLevel,
    #[serde(rename = "explicit_content_filter")]
    pub explicit_content_filter_level: ExplicitContentFilterLevel,

    pub roles: Option<Vec<Role>>,
    pub emojis: Option<Vec<Emoji>>,
    pub stickers: Option<Vec<Sticker>>,
    pub features: Vec<String>,

    // wtf is this used for
    pub application_id: Option<Snowflake>,
    
    pub welcome_screen: Option<WelcomeScreen>,
    
    pub public_updates_channel_id: Option<Snowflake>,
    pub safety_alerts_channel_id: Option<Snowflake>,
    pub system_channel_id: Option<Snowflake>,
    pub rules_channel_id: Option<Snowflake>,
    pub system_channel_flags: u8,
    
    pub afk_channel_id: Option<Snowflake>,
    pub afk_timeout: i32, // in seconds

    pub widget_channel_id: Option<Snowflake>,
    pub widget_enabled: Option<bool>,
    
    pub max_presences: Option<u64>, // almost always null
    pub max_members: Option<u64>,

    #[serde(rename = "premium_tier")]
    pub boost_tier: u8, // 0..=3
    #[serde(rename = "premium_subscription_count")]
    pub boosts: Option<u64>,
    #[serde(rename = "premium_progress_bar_enabled")]
    pub boost_bar_enabled: bool,

    pub max_video_channel_users: u64,
    pub max_stage_video_channel_users: u64,

    // if using "Get current user guilds" endpoint
    pub owner: Option<bool>,
    pub permissions: Option<String>,

    // these need "with_counts" enabled
    pub member_count: Option<u64>,
    pub presence_count: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedGuild {
    pub id: Snowflake,
    pub joined_at: String,

    pub properties: Guild, // Partial
    
    pub large: bool,
    pub lazy: bool,
    
    pub member_count: u64,
    #[serde(rename = "premium_subscription_count")]
    pub boosts: u64,
    
    #[serde(skip)]
    pub activity_instances: Option<()>, // TODO
    #[serde(skip)]
    pub application_command_counts: Option<()>, // TODO
    
    #[serde(skip)]
    pub stage_instances: Vec<()>, // TODO
    pub channels: Vec<Channel>,
    pub threads: Vec<Channel>,
    pub stickers: Vec<Sticker>,
    pub emojis: Vec<Emoji>,
    pub roles: Vec<Role>,
    
    pub data_mode: String,

    #[serde(skip)]
    pub guild_scheduled_events: Vec<()>, // TODO

    pub version: u64,
}

