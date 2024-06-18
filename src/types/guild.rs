use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::sync::MutexGuard;
use crate::context::{Context, Response};
use crate::error::Error;
use crate::types::timestamp::Timestamp;
use crate::types::user::{AvatarDecorationData, User};

use super::{channel::{Channel, welcome_screen::WelcomeScreen}, common::Emoji, role::Role, sticker::Sticker, Snowflake};


#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum MFALevel {
    None = 0, // dont
    Elevated = 1, // need 2FA for moderation
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum VerificationLevel {
    None = 0,     // unrestricted
    Low = 1,      // verified email
    Medium = 2,   // registered for > 5min
    High = 3,     // member of server > 10min
    VeryHigh = 4, // verified phone #
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum MessageNotificationLevel {
    AllMessages = 0,
    OnlyMentions = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
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

impl Guild {
    pub fn icon_url(&self) -> Option<String> {
        self.icon_hash
            .as_ref()
            .map(|hash| format!("https://cdn.discordapp.com/icons/{}/{}.png", self.id, hash))
    }

    pub fn banner_url(&self) -> Option<String> {
        self.banner
            .as_ref()
            .map(|hash| format!("https://cdn.discordapp.com/banners/{}/{}.png", self.id, hash))
    }

    pub fn splash_url(&self) -> Option<String> {
        self.splash
            .as_ref()
            .map(|hash| format!("https://cdn.discordapp.com/splashes/{}/{}.png", self.id, hash))
    }

    pub fn discovery_splash_url(&self) -> Option<String> {
        self.splash_discovery
            .as_ref()
            .map(|hash| format!("https://cdn.discordapp.com/splashes/{}/{}.png", self.id, hash))
    }

    pub async fn fetch_guild(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}", id), None).await
    }

    // Very rarely likely to work
    pub async fn delete_guild(self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        ctx.request(http::Method::DELETE, &format!("/v9/guilds/{}", self.id), None).await
    }

    /**
     * Channels
     */

    pub async fn fetch_channels_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/channels", id), None).await
    }
    pub async fn fetch_channels(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::fetch_channels_static(ctx, self.id).await
    }

    pub async fn create_channel_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, channel: Value) -> anyhow::Result<Response> {
        ctx.request(http::Method::POST, &format!("/v9/guilds/{}/channels", id), Some(channel)).await
    }
    pub async fn create_channel(&self, ctx: &mut MutexGuard<'_, Context>, channel: Value) -> anyhow::Result<Response> {
        Self::create_channel_static(ctx, self.id, channel).await
    }

    /**
     * Members
     */

    pub async fn fetch_members_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, limit: u64) -> anyhow::Result<Response> {
        if limit == 0 || limit > 1000 {
            return Err(Error::InvalidApiRequest("limit must be between 1 and 1000".to_string()).into());
        }
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/members?limit={}", id, limit), None).await
    }
    pub async fn fetch_members(&self, ctx: &mut MutexGuard<'_, Context>, limit: u64) -> anyhow::Result<Response> {
        Self::fetch_members_static(ctx, self.id, limit).await
    }

    pub async fn fetch_member_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, user_id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/members/{}", id, user_id), None).await
    }
    pub async fn fetch_member(&self, ctx: &mut MutexGuard<'_, Context>, user_id: Snowflake) -> anyhow::Result<Response> {
        Self::fetch_member_static(ctx, self.id, user_id).await
    }

    pub async fn search_members_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, query: String, limit: u64) -> anyhow::Result<Response> {
        if limit == 0 || limit > 1000 {
            return Err(Error::InvalidApiRequest("limit must be between 1 and 1000".to_string()).into());
        }
        ctx.request(
            http::Method::GET,
            &format!("/v9/guilds/{}/members/search?query={}&limit={}", id, query, limit),
            None).await
    }
    pub async fn search_members(&self, ctx: &mut MutexGuard<'_, Context>, query: String, limit: u64) -> anyhow::Result<Response> {
        Self::search_members_static(ctx, self.id, query, limit).await
    }

    pub async fn edit_member_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, user_id: Snowflake, member: Value) -> anyhow::Result<Response> {
        ctx.request(http::Method::PATCH, &format!("/v9/guilds/{}/members/{}", id, user_id), Some(member)).await
    }
    pub async fn edit_member(&self, ctx: &mut MutexGuard<'_, Context>, user_id: Snowflake, member: Value) -> anyhow::Result<Response> {
        Self::edit_member_static(ctx, self.id, user_id, member).await
    }

    pub async fn kick_member_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, user_id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::DELETE, &format!("/v9/guilds/{}/members/{}", id, user_id), None).await
    }
    pub async fn kick_member(&self, ctx: &mut MutexGuard<'_, Context>, user_id: Snowflake) -> anyhow::Result<Response> {
        Self::kick_member_static(ctx, self.id, user_id).await
    }

    /**
     * Bans
     */

    pub async fn ban_member_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, user_id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::PUT, &format!("/v9/guilds/{}/bans/{}", id, user_id), None).await
    }
    pub async fn ban_member(&self, ctx: &mut MutexGuard<'_, Context>, user_id: Snowflake) -> anyhow::Result<Response> {
        Self::ban_member_static(ctx, self.id, user_id).await
    }

    pub async fn unban_member_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, user_id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::DELETE, &format!("/v9/guilds/{}/bans/{}", id, user_id), None).await
    }
    pub async fn unban_member(&self, ctx: &mut MutexGuard<'_, Context>, user_id: Snowflake) -> anyhow::Result<Response> {
        Self::unban_member_static(ctx, self.id, user_id).await
    }

    pub async fn get_bans_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/bans", id), None).await
    }
    pub async fn get_bans(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::get_bans_static(ctx, self.id).await
    }

    pub async fn get_ban_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, user_id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/bans/{}", id, user_id), None).await
    }
    pub async fn get_ban(&self, ctx: &mut MutexGuard<'_, Context>, user_id: Snowflake) -> anyhow::Result<Response> {
        Self::get_ban_static(ctx, self.id, user_id).await
    }

    /**
     * Roles
     */

    pub async fn fetch_roles_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/roles", id), None).await
    }
    pub async fn fetch_roles(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::fetch_roles_static(ctx, self.id).await
    }

    pub async fn create_role_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, role: Value) -> anyhow::Result<Response> {
        ctx.request(http::Method::POST, &format!("/v9/guilds/{}/roles", id), Some(role)).await
    }
    pub async fn create_role(&self, ctx: &mut MutexGuard<'_, Context>, role: Value) -> anyhow::Result<Response> {
        Self::create_role_static(ctx, self.id, role).await
    }

    pub async fn edit_role_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, role_id: Snowflake, role: Value) -> anyhow::Result<Response> {
        ctx.request(http::Method::PATCH, &format!("/v9/guilds/{}/roles/{}", id, role_id), Some(role)).await
    }
    pub async fn edit_role(&self, ctx: &mut MutexGuard<'_, Context>, role_id: Snowflake, role: Value) -> anyhow::Result<Response> {
        Self::edit_role_static(ctx, self.id, role_id, role).await
    }

    pub async fn delete_role_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, role_id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::DELETE, &format!("/v9/guilds/{}/roles/{}", id, role_id), None).await
    }
    pub async fn delete_role(&self, ctx: &mut MutexGuard<'_, Context>, role_id: Snowflake) -> anyhow::Result<Response> {
        Self::delete_role_static(ctx, self.id, role_id).await
    }

    pub async fn edit_role_positions_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake, roles: Value) -> anyhow::Result<Response> {
        ctx.request(http::Method::PATCH, &format!("/v9/guilds/{}/roles", id), Some(roles)).await
    }
    pub async fn edit_role_positions(&self, ctx: &mut MutexGuard<'_, Context>, roles: Value) -> anyhow::Result<Response> {
        Self::edit_role_positions_static(ctx, self.id, roles).await
    }

    /**
     * Misc.
     */

    pub async fn fetch_invites_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/invites", id), None).await
    }
    pub async fn fetch_invites(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::fetch_invites_static(ctx, self.id).await
    }

    pub async fn fetch_welcome_screen_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/welcome-screen", id), None).await
    }
    pub async fn fetch_welcome_screen(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::fetch_welcome_screen_static(ctx, self.id).await
    }

    pub async fn fetch_onboarding_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/onboarding", id), None).await
    }
    pub async fn fetch_onboarding(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::fetch_onboarding_static(ctx, self.id).await
    }

    pub async fn fetch_vanity_url_static(ctx: &mut MutexGuard<'_, Context>, id: Snowflake) -> anyhow::Result<Response> {
        ctx.request(http::Method::GET, &format!("/v9/guilds/{}/vanity-url", id), None).await
    }
    pub async fn fetch_vanity_url(&self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Response> {
        Self::fetch_vanity_url_static(ctx, self.id).await
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedGuild {
    pub id: Snowflake,
    pub joined_at: Timestamp,

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

impl CachedGuild {
    pub async fn into_guild(self, ctx: &mut MutexGuard<'_, Context>) -> anyhow::Result<Guild> {
        Ok(serde_json::from_value(Guild::fetch_guild(ctx, self.id).await?.body)?)
    }
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum GuildMemberFlags {
    DidRejoin               = 1 << 0,
    CompletedOnboarding     = 1 << 1,
    BypassesVerification    = 1 << 2,
    StartedOnboarding       = 1 << 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMember {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<Snowflake>,
    pub joined_at: Timestamp,
    pub premium_since: Option<Timestamp>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: GuildMemberFlags,
    pub pending: bool,
    pub permissions: String,
    pub communication_disabled_until: Option<Timestamp>,
    pub avatar_decoration_data: Option<AvatarDecorationData>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMemberEditBuilder {
    value: Value,
}

impl GuildMemberEditBuilder {
    pub fn new() -> Self {
        Self {
            value: json!({})
        }
    }

    pub fn set_nick(&mut self, nick: String) {
        self.value["nick"] = json!(nick);
    }

    pub fn set_roles(&mut self, roles: Vec<Snowflake>) {
        self.value["roles"] = json!(roles);
    }

    pub fn set_deaf(&mut self, deaf: bool) {
        self.value["deaf"] = json!(deaf);
    }

    pub fn set_mute(&mut self, mute: bool) {
        self.value["mute"] = json!(mute);
    }

    pub fn set_voice_channel(&mut self, channel_id: Snowflake) {
        self.value["channel_id"] = json!(channel_id);
    }

    pub fn set_timeout(&mut self, timeout: Option<String>) {
        self.value["communication_disabled_until"] = json!(timeout);
    }

    pub fn build(self) -> Value {
        self.value
    }
}

