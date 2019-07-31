#[macro_use]
extern crate clap;
use clap::App;
use kvs::KvStore;
use std::process;
use std::path::Path;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    let mut store = KvStore::new();

    if let Some(matches) = matches.subcommand_matches("get") {
        if matches.is_present("INPUT") {
            println!("Printing debug info...");
            let key = matches.value_of("INPUT").unwrap_or("default.conf");
            println!("Value for key: {}", key.to_string());

            match store.get(key.to_string()) {
                Ok(v) => println!("{:?}", v),
                _ => {
                    eprintln!("unimplemented");
                    process::exit(1);
                }
            }
        } else {
            println!("Printing normally...");
        }
    }
    if let Some(matches) = matches.subcommand_matches("rm") {
        if matches.is_present("INPUT") {
            println!("Printing debug info...");
            let key = matches.value_of("INPUT").unwrap_or("default.conf");
            println!("Value for key: {}", key.to_string());

            match store.get(key.to_string()) {
                Ok(v) => println!("{:?}", v),
                _ => {
                    eprintln!("unimplemented");
                    process::exit(1);
                }
            }
        } else {
            println!("Printing normally...");
        }
    }

    if let Some(matches) = matches.subcommand_matches("set") {
        if matches.is_present("INPUT") {
            println!("Printing debug info...");
            let key = matches.value_of("INPUT").unwrap_or("default.conf");
            println!("Value for key: {}", key.to_string());
            let value = matches.value_of("VALUE").unwrap_or("default.conf");
            store.set(key.to_string(), value.to_string());
        } else {
            println!("Printing normally...");
        }
    }

    process::exit(1);
}
