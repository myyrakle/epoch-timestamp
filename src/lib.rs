use std::time::{SystemTime, UNIX_EPOCH};

pub struct Epoch {}

impl Epoch {
    pub fn now() -> u64 {
        let now = SystemTime::now();
        let epoch = now.duration_since(UNIX_EPOCH).unwrap();

        epoch.as_secs()
    }

    pub fn second(number: u64) -> u64 {
        number
    }

    pub fn minute(number: u64) -> u64 {
        Self::second(number) * 60
    }

    pub fn hour(number: u64) -> u64 {
        Self::minute(number) * 60
    }

    pub fn day(number: u64) -> u64 {
        Self::hour(number) * 24
    }

    pub fn week(number: u64) -> u64 {
        Self::day(number) * 7
    }

    pub fn year(number: u64) -> u64 {
        Self::day(number) * 365
    }
}
