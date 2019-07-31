use std::collections::HashMap;
use std::process;
pub use crate::{KvsError, Result};
use std::path::Path;
use std::fs;
use std::io::Write;
use serde::{Serialize, Deserialize};


pub struct KvStore {
    store: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Log(String, String);

impl KvStore {
    pub fn new() -> Self {
        let store = HashMap::new();
        Self { store }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let point = Log(key, value);

            // Convert the Point to a JSON string.
        let serialized = serde_json::to_string(&point).unwrap();
           
                    // Prints serialized = {"x":1,"y":2}
        println!("serialized = {}", serialized);
            
                             // Convert the JSON string back to a Point.
            //                     let deserialized: Point =
        //serde_json::from_str(&serialized).unwrap();
            

        let mut log = fs::File::create("log").unwrap();

        match log.write(serialized.as_bytes()) {
            Ok(_) => process::exit(0),
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            },
        };
                                         
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
