use std::time::{SystemTime, UNIX_EPOCH};

pub fn make_timestamp() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => 0,
    }
    // TODO: panic if there is an error creating the timestamp
}
