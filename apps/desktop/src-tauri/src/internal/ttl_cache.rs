use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct TtlCache<K, V> {
    entries: HashMap<K, (V, Instant)>,
    ttl: Duration,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> TtlCache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: HashMap::new(),
            ttl,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.entries.insert(key, (value, Instant::now()));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.entries.remove(key).map(|(v, _)| v)
    }

    pub fn purge_expired(&mut self) {
        let now = Instant::now();

        self.entries
            .retain(|_, (_, instant)| now.duration_since(*instant) < self.ttl);
    }
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Default for TtlCache<K, V> {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}
