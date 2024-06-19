use crate::types::Snowflake;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Timestamp(pub DateTime<FixedOffset>);

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_rfc3339())
    }
}

impl From<DateTime<FixedOffset>> for Timestamp {
    fn from(value: DateTime<FixedOffset>) -> Self {
        Self(value)
    }
}

impl From<Timestamp> for DateTime<FixedOffset> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl From<Timestamp> for String {
    fn from(value: Timestamp) -> Self {
        value.0.to_rfc3339()
    }
}

impl From<String> for Timestamp {
    fn from(value: String) -> Self {
        Self(value.parse().expect("Couldn't parse timestamp from string"))
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map(Timestamp)
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0.to_rfc3339())
    }
}

impl From<Snowflake> for Timestamp {
    fn from(value: Snowflake) -> Self {
        let ms = value.timestamp();
        Self(
            DateTime::from_timestamp(ms as i64 / 1000, (ms % 1000) as u32 * 1_000_000)
                .unwrap()
                .into(),
        )
    }
}
