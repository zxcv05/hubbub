use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::common::Emoji;

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum ActivityType {
    Playing = 0,      // Playing {name}
    Steaming = 1,  // Streaming {details}
    Listening = 2, // Listening to {name}
    Watching = 3,  // Watching {name}
    Custom = 4,    // {emoji} {state}
    Competing
    = 5, // Competing in {name}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Timestamps {
    pub start: Option<u64>, // ms
    pub end: Option<u64>,   // ms
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Party {
    pub id: Option<String>,
    pub size: Option<[u64; 2]>, // [current_size, max_size],
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Asset {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Secrets {
    #[serde(rename = "join")]
    pub join_key: Option<String>,
    #[serde(rename = "spectate")]
    pub spectate_key: Option<String>,
    #[serde(rename = "match")]
    pub match_key: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u16)]
pub enum ActivityFlag {
    Instance = 1 << 0,
    Join = 1 << 1,
    Spectate = 1 << 2,
    JoinReq = 1 << 3,
    Sync = 1 << 4,
    Play = 1 << 5,
    PrivacyFriends = 1 << 6,
    PrivacyVC = 1 << 7,
    Embedded = 1 << 8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Button {
    pub label: String, // 1-32 limit
    pub url: String,   // 1-512 limit
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: ActivityType,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<Timestamps>,
    pub application_id: u64, // Snowflake
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<Asset>,
    pub secrets: Option<Secrets>,
    pub instance: Option<bool>,
    pub buttons: Option<Vec<Button>>,
    pub flags: u16,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ActivityBuilder {
    value: Value,
}

impl ActivityBuilder {
    pub fn new(name: String, activity_type: ActivityType) -> Self {
        Self {
            value: json!({
                "name": name,
                "type": activity_type
            }),
        }
    }

    pub fn set_url(mut self, url: String) -> Self {
        if url.len() > 512 {
            panic!("URL must be less than 512 characters");
        }
        self.value["url"] = json!(url);
        self
    }

    pub fn set_timestamps(mut self, start: u64, end: u64) -> Self {
        self.value["timestamps"] = json!({
            "start": start,
            "end": end
        });
        self
    }

    pub fn set_application_id(mut self, application_id: u64) -> Self {
        self.value["application_id"] = json!(application_id);
        self
    }

    pub fn set_emoji(mut self, emoji: Emoji) -> Self {
        self.value["emoji"] = json!(emoji);
        self
    }

    pub fn set_asset(mut self, assets: Asset) -> Self {
        self.value["assets"] = json!(assets);
        self
    }

    pub fn set_flags(mut self, flags: u16) -> Self {
        self.value["flags"] = json!(flags);
        self
    }

    pub fn set_state(mut self, state: String) -> Self {
        self.value["state"] = json!(state);
        self
    }

    pub fn add_button(mut self, label: String, url: String) -> Self {
        let buttons = match self.value["buttons"].as_array_mut() {
            Some(v) => v,
            None => &mut Vec::new(),
        };

        if label.len() > 32 {
            panic!("Button label must be less than 32 characters");
        }

        if url.len() > 512 {
            panic!("Button url must be less than 512 characters");
        }

        if buttons.len() == 2 {
            panic!("Can't have more than 2 buttons in one activity");
        }

        buttons.push(json!({
            "label": label,
            "url": url
        }));
        self.value["buttons"] = json!(buttons);
        self
    }

    pub fn build(self) -> Value {
        self.value
    }
}
