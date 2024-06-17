use std::fmt::Display;
use std::str::FromStr;

use serde::{de, Deserialize, Deserializer, Serialize};

use crate::prelude::Error;


pub static DISCORD_EPOCH: u128 = 1420070400000;


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Hash)]
pub struct Snowflake(u64);

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<String> for Snowflake {
    fn from(value: String) -> Self {
        Self(u64::from_str_radix(value.as_str(), 10).expect("Couldn't parse snowflake from string"))
    }
}

impl FromStr for Snowflake {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u64::from_str_radix(s, 10).expect("Couldn't parse snowflake from string")))
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        String::deserialize(deserializer)?.parse().map_err(de::Error::custom)
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.collect_str(&self.0.to_string())
    }
}

impl Snowflake {
    pub fn timestamp(&self) -> u128 {
        (self.0 as u128 >> 22) + DISCORD_EPOCH
    }

    pub fn worker_id(&self) -> u8 {
        ((self.0 & 0x3E0000) >> 17) as u8
    }

    pub fn process_id(&self) -> u8 {
        ((self.0 & 0x1F000) >> 12) as u8
    }

    pub fn increment(&self) -> u16 {
        (self.0 & 0xFFF) as u16
    }
}


