use std::collections::HashMap;
use std::process;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        let mut store = HashMap::new();
        Self { store }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.store.get(&key).cloned()
    }
}
