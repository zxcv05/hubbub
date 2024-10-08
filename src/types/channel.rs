use crate::context::{Context, Response};
use crate::error::Error;
use crate::types::timestamp::Timestamp;

use anyhow::Result;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tokio::sync::MutexGuard;

use super::{user::User, Snowflake};

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum ChannelType {
    Text = 0,
    DM = 1,
    Voice = 2,
    Group = 3,
    Category = 4,
    Announcement = 5,
    ThreadAnnouncement = 10,
    ThreadPublic = 11,
    ThreadPrivate = 12,
    VoiceStage = 13,
    Directory = 14,
    Forum = 15,
    Media = 16,
}

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u16)]
pub enum ChannelFlag {
    Pinned = 1 << 1,
    RequireTag = 1 << 4,
    HideDownloadOptions = 1 << 15,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum VideoQualityMode {
    Auto = 1, // "Not present"
    Full = 2, // 720p
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64, // in minutes
    pub archive_timestamp: Timestamp,
    pub locked: bool,

    // Threads > 2022-01-09
    pub create_timestamp: Option<Timestamp>,

    // Private threads
    pub invitable: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ThreadMember {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,

    pub join_timestamp: Timestamp,
    pub flags: u64, // notifications
                    // pub member: Option<GuildMember>,
}

#[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PermissionOverwrite {
    pub id: Snowflake,
    pub allow: String, // TODO serialize into u128
    pub deny: String,  // TODO serialize into u128
    #[serde(rename = "type")]
    pub overwrite_type: u8,
}

fn default_spam_value() -> bool {
    false
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Channel {
    pub id: Snowflake,
    pub name: Option<String>,
    pub topic: Option<String>,

    pub nsfw: Option<bool>,

    pub position: Option<u64>,

    // VC
    pub bitrate: Option<u64>, // in bits, obviously
    pub user_limit: Option<u64>,
    pub rate_limit_per_user: Option<u64>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,

    // DM
    #[serde(rename = "recipient_ids")]
    pub users_ids: Option<Vec<Snowflake>>,

    #[serde(rename = "recipients")]
    pub users: Option<Vec<User>>,

    // Group DM
    pub icon: Option<String>,
    pub application_id: Option<Snowflake>,
    pub managed: Option<bool>,

    // Group DM or thread
    pub owner_id: Option<Snowflake>,

    // Thread
    pub message_count: Option<u64>, // doesnt count first msg
    pub member_count: Option<u64>,  // stops counting at 50
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>, // only certain apis include this

    // Guild channels
    pub guild_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>, // category id

    pub last_message_id: Option<String>,
    pub last_pin_timestamp: Option<Timestamp>,

    #[serde(rename = "safety_warnings")]
    pub warnings: Option<Vec<String>>,

    #[serde(default = "default_spam_value")]
    pub is_spam: bool,

    pub permission_overwrites: Option<Vec<PermissionOverwrite>>,

    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub flags: u64,
}

impl Channel {
    pub fn is_dm(&self) -> bool {
        self.channel_type == ChannelType::DM || self.channel_type == ChannelType::Group
    }

    pub fn is_thread(&self) -> bool {
        self.channel_type == ChannelType::ThreadAnnouncement
            || self.channel_type == ChannelType::ThreadPrivate
            || self.channel_type == ChannelType::ThreadPublic
    }

    pub fn is_forum(&self) -> bool {
        self.channel_type == ChannelType::Forum || self.channel_type == ChannelType::Media
    }

    pub fn is_voice(&self) -> bool {
        self.channel_type == ChannelType::Voice || self.channel_type == ChannelType::VoiceStage
    }

    pub fn is_text(&self) -> bool {
        self.channel_type == ChannelType::Text || self.is_dm()
    }

    pub fn is_category(&self) -> bool {
        self.channel_type == ChannelType::Category
    }

    /**
     * Channel
     */

    pub async fn fetch_channel(
        ctx: &mut MutexGuard<'_, Context>,
        id: Snowflake,
    ) -> Result<Response> {
        ctx.request(Method::GET, &format!("/v9/channels/{}", id), None)
            .await
    }

    pub async fn delete_channel(
        self,
        ctx: &mut MutexGuard<'_, Context>,
    ) -> Result<Response> {
        Self::delete_channel_static(ctx, self.id).await
    }
    pub async fn delete_channel_static(
        ctx: &mut MutexGuard<'_, Context>,
        id: Snowflake,
    ) -> Result<Response> {
        ctx.request(Method::DELETE, &format!("/v9/channels/{}", id), None)
            .await
    }

    /**
     * Message
     */

    pub async fn send_message(
        &self,
        ctx: &mut MutexGuard<'_, Context>,
        msg: Value,
    ) -> Result<Response> {
        Self::send_message_static(ctx, self.id, msg).await
    }
    pub async fn send_message_static(
        ctx: &mut MutexGuard<'_, Context>,
        channel_id: Snowflake,
        msg: Value,
    ) -> Result<Response> {
        ctx.request(
            Method::POST,
            &format!("/v9/channels/{}/messages", channel_id),
            Some(msg),
        )
        .await
    }

    pub async fn fetch_messages(
        &self,
        ctx: &mut MutexGuard<'_, Context>,
        limit: u64,
        before: Option<Snowflake>,
    ) -> Result<Response> {
        Self::fetch_messages_static(ctx, self.id, limit, before).await
    }
    pub async fn fetch_messages_static(
        ctx: &mut MutexGuard<'_, Context>,
        channel_id: Snowflake,
        limit: u64,
        before: Option<Snowflake>,
    ) -> Result<Response> {
        if limit > 100 {
            return Err(Error::InvalidApiRequest("limit must be less than 100".to_string()).into());
        }

        let mut endpoint = format!("/v9/channels/{}/messages?limit={}", channel_id, limit);

        if let Some(before) = before {
            endpoint.push_str(&format!("&before={}", before));
        }

        ctx.request(Method::GET, &endpoint, None).await
    }

    pub async fn fetch_message(
        &self,
        ctx: &mut MutexGuard<'_, Context>,
        id: Snowflake,
    ) -> Result<Response> {
        Self::fetch_message_static(ctx, self.id, id).await
    }
    pub async fn fetch_message_static(
        ctx: &mut MutexGuard<'_, Context>,
        channel_id: Snowflake,
        id: Snowflake,
    ) -> Result<Response> {
        ctx.request(
            Method::GET,
            &format!("/v9/channels/{}/messages/{}", channel_id, id),
            None,
        )
        .await
    }

    pub async fn delete_message(
        self,
        ctx: &mut MutexGuard<'_, Context>,
        id: Snowflake,
    ) -> Result<Response> {
        Self::delete_message_static(ctx, self.id, id).await
    }
    pub async fn delete_message_static(
        ctx: &mut MutexGuard<'_, Context>,
        channel_id: Snowflake,
        id: Snowflake,
    ) -> Result<Response> {
        ctx.request(
            Method::DELETE,
            &format!("/v9/channels/{}/messages/{}", channel_id, id),
            None,
        )
        .await
    }

    pub async fn edit_message(
        self,
        ctx: &mut MutexGuard<'_, Context>,
        id: Snowflake,
        msg: Value
    ) -> Result<Response> {
        Self::edit_message_static(ctx, self.id, id, msg).await
    }
    pub async fn edit_message_static(
        ctx: &mut MutexGuard<'_, Context>,
        channel_id: Snowflake,
        id: Snowflake,
        msg: Value
    ) -> Result<Response> {
        ctx.request(
            Method::PATCH,
            &format!("/v9/channels/{}/messages/{}", channel_id, id),
            Some(msg)
        )
        .await
    }

    pub async fn add_reaction(
        self,
        ctx: &mut MutexGuard<'_, Context>,
        id: Snowflake,
        emoji: &str
    ) -> Result<Response> {
        Self::add_reaction_static(ctx, self.id, id, emoji).await
    }
    pub async fn add_reaction_static(
        ctx: &mut MutexGuard<'_, Context>,
        channel_id: Snowflake,
        id: Snowflake,
        emoji: &str
    ) -> Result<Response> {
        ctx.request(
            Method::PUT,
            &format!("/v9/channels/{}/messages/{}/reactions/{}/@me", channel_id, id, emoji),
            None
        )
        .await
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChannelMention {
    pub id: Snowflake,
    pub guild_id: Snowflake,

    pub name: String,

    #[serde(rename = "type")]
    pub channel_type: ChannelType,
}

pub mod welcome_screen {
    use serde::{Deserialize, Serialize};

    use crate::types::Snowflake;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct WelcomeScreen {
        pub description: Option<String>,
        #[serde(rename = "welcome_channels")]
        pub channels: Vec<WelcomeChannel>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct WelcomeChannel {
        pub channel_id: Snowflake,
        pub description: String,

        pub emoji_id: Option<Snowflake>,
        pub emoji_name: Option<String>,
    }
}

pub mod stage_instance {
    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    use crate::types::Snowflake;

    #[derive(Deserialize_repr, Serialize_repr, Debug, Eq, PartialEq, Clone)]
    #[repr(u8)]
    pub enum PrivacyLevel {
        Public = 1,
        GuildOnly = 2,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    pub struct StageInstance {
        pub id: Snowflake,
        pub guild_id: Snowflake,
        pub channel_id: Snowflake,
        pub topic: String,
        pub privacy_level: PrivacyLevel,
        pub guild_scheduled_event_id: Option<Snowflake>,
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChannelBuilder {
    value: Value,
}

impl ChannelBuilder {
    pub fn new(name: String, channel_type: ChannelType) -> Self {
        Self {
            value: json!({
                "name": name,
                "type": channel_type
            }),
        }
    }

    pub fn set_topic(&mut self, topic: String) {
        self.value["topic"] = json!(topic);
    }

    pub fn set_nsfw(&mut self, nsfw: bool) {
        self.value["nsfw"] = json!(nsfw);
    }

    pub fn set_position(&mut self, position: u64) {
        self.value["position"] = json!(position);
    }

    pub fn set_bitrate(&mut self, bitrate: u64) {
        self.value["bitrate"] = json!(bitrate);
    }

    pub fn set_user_limit(&mut self, user_limit: u64) {
        self.value["user_limit"] = json!(user_limit);
    }

    pub fn set_rate_limit_per_user(&mut self, rate_limit_per_user: u64) {
        self.value["rate_limit_per_user"] = json!(rate_limit_per_user);
    }

    pub fn add_permission_overwrite(&mut self, overwrite: Value) {
        let permission_overwrites = match self.value["permission_overwrites"].as_array_mut() {
            Some(v) => v,
            None => &mut Vec::new(),
        };

        permission_overwrites.push(overwrite);
        self.value["permission_overwrites"] = json!(permission_overwrites);
    }

    pub fn set_parent_category(&mut self, parent_id: Snowflake) {
        self.value["parent_id"] = json!(parent_id);
    }

    pub fn set_rtc_region(&mut self, rtc_region: String) {
        self.value["rtc_region"] = json!(rtc_region);
    }

    pub fn build(self) -> Value {
        self.value
    }
}
