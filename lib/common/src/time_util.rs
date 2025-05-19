use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn unix_timestamp() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

pub fn unix_timestamp_seconds() -> i64 {
    unix_timestamp().as_secs() as i64
}

pub fn unix_timestamp_ms() -> u64 {
    unix_timestamp().as_millis() as u64
}
