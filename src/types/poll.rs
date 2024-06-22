use crate::types::common::Emoji;
use crate::types::timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Media {
    pub text: Option<String>,
    pub emoji: Option<Emoji>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Answer {
    pub answer_id: Option<u64>, // dont ever send
    pub poll_media: Media,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AnswerCount {
    pub id: u64,
    pub count: u64,
    pub me_voted: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Eq, PartialEq, Clone)]
#[repr(u8)]
pub enum LayoutType {
    Default = 1,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Results {
    pub is_finalized: bool,
    pub answer_counts: Vec<AnswerCount>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Poll {
    pub question: Media,
    pub answers: Vec<Answer>,
    pub expiry: Option<Timestamp>,
    pub allow_multiselect: bool,
    pub layout_type: LayoutType,
    pub results: Option<Results>,
}
