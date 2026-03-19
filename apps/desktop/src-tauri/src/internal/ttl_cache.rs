use std::collections::HashMap;
use std::time::{Duration, Instant};

pub struct TtlCache<K, V> {
    map: HashMap<K, (V, Instant)>,
    ttl: Duration,
}

impl<K: std::cmp::Eq + std::hash::Hash, V> TtlCache<K, V> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            map: HashMap::new(),
            ttl,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, (value, Instant::now()));
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(key).map(|(v, _)| v)
    }

    pub fn tidy(&mut self) {
        let now = Instant::now();

        // @todo-now does the other values get droped?

        self.map
            .retain(|_, (_, instant)| now.duration_since(*instant) < self.ttl);
    }
}

impl<K: std::cmp::Eq + std::hash::Hash, V> Default for TtlCache<K, V> {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}
