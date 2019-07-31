use std::collections::HashMap;
use std::process;
pub use crate::{KvsError, Result};
use std::path::Path;

pub struct KvStore {
    store: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        let store = HashMap::new();
        Self { store }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        unimplemented!();
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
      unimplemented!();
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        let key : &str = "aze";
        unimplemented!();
    }
    pub fn open(path: &Path) -> Result<(Self)> {
        unimplemented!();
    }
}
