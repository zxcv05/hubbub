use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{user::User, Snowflake};


#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum StickerType {
    Standard = 1, // "Official" sticker
    Guild = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum StickerFormat {
    PNG = 1,
    APNG = 2,
    LOTTIE = 3,
    GIF = 4,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerItem {
    pub id: Snowflake,
    pub name: String,
    pub format_type: StickerFormat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    pub id: Snowflake,
    pub pack_id: Option<Snowflake>,
    pub guild_id: Snowflake,

    pub name: String,
    pub description: Option<String>,

    pub tags: String, // comma separated

    #[serde(rename = "type")]
    pub sticker_type: StickerType,
    pub format_type: StickerFormat,

    #[serde(rename = "user")]
    pub creator: Option<User>,
    
    pub available: Option<bool>,
    pub sort_value: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StickerPack {
    pub id: Snowflake,
    pub name: String,

    pub sticker: Vec<Sticker>,

    pub sku_id: Snowflake,
    pub cover_sticker_id: Option<Snowflake>,
    pub banner_asset_id: Option<Snowflake>,

    pub description: String,
}
