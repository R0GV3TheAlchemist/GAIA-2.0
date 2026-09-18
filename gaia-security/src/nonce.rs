//! Sliding-window nonce store. In-process only. Author: Kyle Steen
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayResult {
    Fresh,
    Replayed,
}

struct NonceRecord {
    admitted_at: Instant,
}

pub struct NonceStore {
    window: Duration,
    records: HashMap<(String, String), NonceRecord>,
    evict_interval: usize,
    admit_count: usize,
}

impl NonceStore {
    pub fn new(window: Duration) -> Self {
        Self {
            window,
            records: HashMap::new(),
            evict_interval: 1000,
            admit_count: 0,
        }
    }

    pub fn with_evict_interval(window: Duration, evict_interval: usize) -> Self {
        Self {
            window,
            records: HashMap::new(),
            evict_interval,
            admit_count: 0,
        }
    }

    pub fn admit(&mut self, key_id: &str, nonce: &str) -> ReplayResult {
        self.admit_count += 1;
        if self.evict_interval > 0 && self.admit_count % self.evict_interval == 0 {
            self.evict_expired();
        }
        let key = (key_id.to_owned(), nonce.to_owned());
        if self.records.contains_key(&key) {
            return ReplayResult::Replayed;
        }
        self.records.insert(key, NonceRecord {
            admitted_at: Instant::now(),
        });
        ReplayResult::Fresh
    }

    pub fn evict_expired(&mut self) {
        let window = self.window;
        self.records.retain(|_, v| v.admitted_at.elapsed() < window);
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at08_replay_protection_scenario() {
        let mut store = NonceStore::new(Duration::from_secs(300));
        let key = "ed25519-pubkey-hex-abc123";
        let nonce = "unique-nonce-for-this-request";
        assert_eq!(store.admit(key, nonce), ReplayResult::Fresh);
        assert_eq!(store.admit(key, nonce), ReplayResult::Replayed);
    }

    #[test]
    fn different_nonce_same_key_fresh() {
        let mut store = NonceStore::new(Duration::from_secs(300));
        store.admit("key-1", "nonce-1");
        assert_eq!(store.admit("key-1", "nonce-2"), ReplayResult::Fresh);
    }
}
