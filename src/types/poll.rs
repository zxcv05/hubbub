use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::types::common::Emoji;

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub text: Option<String>,
    pub emoji: Option<Emoji>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub answer_id: Option<u64>, // dont ever send
    pub poll_media: Media,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerCount {
    pub id: u64,
    pub count: u64,
    pub me_voted: bool,
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum LayoutType {
    Default = 1
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    pub is_finalized: bool,
    pub answer_counts: Vec<AnswerCount>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Poll {
    pub question: Media,
    pub answers: Vec<Answer>,
    pub expiry: Option<String>, // ISO8601
    pub allow_multiselect: bool,
    pub layout_type: LayoutType,
    pub results: Option<Results>,
}

