use dashmap::DashMap;
use std::time::{Duration, Instant};

pub struct RateLimiter {
    store: DashMap<String, (u32, Instant)>,
    limit: u32,
}

impl RateLimiter {
    pub fn new(limit: u32) -> Self {
        Self {
            store: DashMap::new(),
            limit,
        }
    }

    pub fn check(&self, ip: String) -> bool {
        let now = Instant::now();
        let mut entry = self.store.entry(ip).or_insert((0, now));

        if now.duration_since(entry.1) > Duration::from_secs(60) {
            *entry = (0, now);
        }

        if entry.0 >= self.limit {
            return false;
        }

        entry.0 += 1;
        true
    }
}
