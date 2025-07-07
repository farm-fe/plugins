use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

/// A simple cache with time-based expiration
pub struct TimedCache<K, V> {
    data: Arc<RwLock<HashMap<K, (V, u64)>>>,
    ttl_seconds: u64,
}

impl<K, V> TimedCache<K, V> 
where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
        }
    }

    /// Get value from cache if it exists and is not expired
    pub fn get(&self, key: &K) -> Option<V> {
        let now = current_timestamp();
        let data = self.data.read().ok()?;
        
        if let Some((value, timestamp)) = data.get(key) {
            if now - timestamp < self.ttl_seconds {
                return Some(value.clone());
            }
        }
        None
    }

    /// Insert value into cache
    pub fn insert(&self, key: K, value: V) {
        let now = current_timestamp();
        if let Ok(mut data) = self.data.write() {
            data.insert(key, (value, now));
        }
    }

    /// Clear expired entries
    pub fn cleanup(&self) {
        let now = current_timestamp();
        if let Ok(mut data) = self.data.write() {
            data.retain(|_, (_, timestamp)| now - *timestamp < self.ttl_seconds);
        }
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.data.read().map(|data| data.len()).unwrap_or(0)
    }
}

/// Get current timestamp in seconds
#[inline]
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Thread-safe LRU cache
pub struct LruCache<K, V> {
    capacity: usize,
    data: Arc<RwLock<HashMap<K, V>>>,
    access_order: Arc<RwLock<Vec<K>>>,
}

impl<K, V> LruCache<K, V>
where 
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            data: Arc::new(RwLock::new(HashMap::new())),
            access_order: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get value from cache
    pub fn get(&self, key: &K) -> Option<V> {
        let value = {
            let data = self.data.read().ok()?;
            data.get(key).cloned()
        };

        if value.is_some() {
            // Update access order
            if let Ok(mut order) = self.access_order.write() {
                if let Some(pos) = order.iter().position(|k| k == key) {
                    order.remove(pos);
                }
                order.push(key.clone());
            }
        }

        value
    }

    /// Insert value into cache
    pub fn insert(&self, key: K, value: V) {
        if let (Ok(mut data), Ok(mut order)) = (self.data.write(), self.access_order.write()) {
            // Remove existing key if present
            if data.contains_key(&key) {
                if let Some(pos) = order.iter().position(|k| k == &key) {
                    order.remove(pos);
                }
            }

            // Check capacity and evict if necessary
            while data.len() >= self.capacity && !order.is_empty() {
                let oldest_key = order.remove(0);
                data.remove(&oldest_key);
            }

            data.insert(key.clone(), value);
            order.push(key);
        }
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.data.read().map(|data| data.len()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timed_cache() {
        let cache = TimedCache::new(1); // 1 second TTL
        
        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        
        thread::sleep(Duration::from_millis(1100));
        assert_eq!(cache.get(&"key1".to_string()), None);
    }

    #[test]
    fn test_lru_cache() {
        let cache = LruCache::new(2);
        
        cache.insert("key1".to_string(), "value1".to_string());
        cache.insert("key2".to_string(), "value2".to_string());
        
        assert_eq!(cache.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(cache.get(&"key2".to_string()), Some("value2".to_string()));
        
        // This should evict key1
        cache.insert("key3".to_string(), "value3".to_string());
        
        assert_eq!(cache.get(&"key1".to_string()), None);
        assert_eq!(cache.get(&"key2".to_string()), Some("value2".to_string()));
        assert_eq!(cache.get(&"key3".to_string()), Some("value3".to_string()));
    }
}