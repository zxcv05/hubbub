use std::{fmt::Display, time::Instant};

use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    NotLoggedIn,
    NoTokenGiven,
    Ratelimit(Instant),
}


impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotLoggedIn => f.write_str("Not logged in"),
            Error::NoTokenGiven => f.write_str("No token given"),
            Error::Ratelimit(i) => f.write_fmt(format_args!("Ratelimited until {i:?}"))
        }
    }
}

