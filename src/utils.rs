use std::time::{SystemTime, UNIX_EPOCH};

pub fn t() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs() as f64 +
            since_the_epoch.subsec_nanos() as f64 / 1_000_000_000.0;
}
