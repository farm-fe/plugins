use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct WorkerCache {
  cache: Arc<Mutex<HashMap<String, String>>>,
}

impl WorkerCache {
  pub fn new() -> Self {
    WorkerCache {
      cache: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn get(&self, key: &str) -> Option<String> {
    let cache = self.cache.lock().unwrap();
    cache.get(key).cloned()
  }

  pub fn insert(&self, key: String, value: String) {
    let mut cache = self.cache.lock().unwrap();
    cache.insert(key, value);
  }
}
