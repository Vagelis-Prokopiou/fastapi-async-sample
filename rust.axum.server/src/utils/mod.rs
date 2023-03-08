#![allow(dead_code)]
use std::time::{SystemTime, UNIX_EPOCH};

pub fn getTimestamp() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_epoch.as_secs();
    return timestamp;
}
