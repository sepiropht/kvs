use std::collections::HashMap;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        let store: HashMap<String, String> = HashMap::new();
        Self { store }
    }
    pub fn remove(&mut self, key: String) -> Option<String> {
        let value = self.get(key.clone());
        self.store.remove(&key);
        value
    }
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
    pub fn get(&self, key: String) -> Option<String> {
        match self.store.get(&key) {
            Some(value) => Some(value.to_owned()),
            _ => None,
        }
    }
}
