use std::process;

pub struct KvStore {
}

impl KvStore {
    pub fn new() -> Self {
        Self {}
    }
    pub fn set(&mut self, key: String, value: String) {
        eprintln!("unimplemented");
        process::exit(1);
        unimplemented!();

    }

    pub fn remove(&mut self, key: String) {
        eprintln!("unimplemented");
        process::exit(1);
        unimplemented!();

    }

    pub fn get(&self, query: String) -> Option<String> {
        eprintln!("unimplemented");
        process::exit(1);
        unimplemented!();
    }
}
