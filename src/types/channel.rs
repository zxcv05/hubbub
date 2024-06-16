use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{user::User, Snowflake};

#[derive(Deserialize_repr, Serialize_repr, Debug)]
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
    HideDownloadOptions = 1 << 15
}

#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum VideoQualityMode {
    Auto = 1, // "Not present"
    Full = 2, // 720p
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u64, // in minutes
    pub archive_timestamp: String, // ISO8601
    pub locked: bool,

    // Threads > 2022-01-09
    pub create_timestamp: Option<String>, // ISO8601

    // Private threads
    pub invitable: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ThreadMember {
    pub id: Option<Snowflake>,
    pub user_id: Option<Snowflake>,
    
    pub join_timestamp: String, // ISO8601
    pub flags: u64, // notifications
    // pub member: Option<GuildMember>,
}


#[derive(Deserialize_repr, Serialize_repr, Debug)]
#[repr(u8)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PermissionOverwrite {
    pub id: Snowflake,
    pub allow: String, // TODO serialize into u128
    pub deny: String, // TODO serialize into u128
    #[serde(rename = "type")]
    pub overwrite_type: u8,
}

fn default_spam_value() -> bool { false }

#[derive(Deserialize, Serialize, Debug)]
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
    pub member_count: Option<u64>, // stops counting at 50
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>, // only certain apis include this

    // Guild channels
    pub guild_id: Option<Snowflake>,
    pub parent_id: Option<Snowflake>, // category id

    pub last_message_id: Option<String>,
    pub last_pin_timestamp: Option<String>, // iso8601
    
    #[serde(rename = "safety_warnings")]
    pub warnings: Option<Vec<String>>,
    
    #[serde(default = "default_spam_value")]
    pub is_spam: bool,

    pub permission_overwrites: Option<Vec<PermissionOverwrite>>, // TODO
    
    #[serde(rename = "type")]
    pub channel_type: ChannelType,
    pub flags: u64,
}


pub mod welcome_screen {
    use serde::{Deserialize, Serialize};

    use crate::types::Snowflake;

    #[derive(Deserialize, Serialize, Debug)]
    pub struct WelcomeScreen {
        pub description: Option<String>,
        #[serde(rename = "welcome_channels")]
        pub channels: Vec<WelcomeChannel>,
    }
    
    #[derive(Deserialize, Serialize, Debug)]
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

    #[derive(Deserialize_repr, Serialize_repr, Debug)]
    #[repr(u8)]
    pub enum PrivacyLevel {
        Public = 1,
        GuildOnly = 2,
    }
    
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct StageInstance {
        pub id: Snowflake,
        pub guild_id: Snowflake,
        pub channel_id: Snowflake,
        pub topic: String,
        pub privacy_level: PrivacyLevel,
        pub guild_scheduled_event_id: Option<Snowflake>,
    }
}

