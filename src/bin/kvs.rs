use kvs::KvStore;
use std::process;
use structopt::StructOpt;
/// An key value
#[derive(StructOpt, Debug)]
#[structopt(name = "kvs")]
/// distributed database
enum Opt {
    /// Set value with key

    #[structopt(help = "set  value and it key")]
    Set {
        #[structopt(short)]
        interactive: bool,
        #[structopt(short)]
        all: bool,
        key_value: Vec<String>,
    },
    #[structopt(help = "set  value and it key")]
    Get {
        #[structopt(short)]
        interactive: bool,
        #[structopt(short)]
        all: bool,
        key: String,
    },
    Rm {
        #[structopt(short)]
        interactive: bool,
        #[structopt(short)]
        all: bool,
        key: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    let mut store = KvStore::new();
    match opt {
        Opt::Set {
            key_value,
            all,
            interactive,
        } => {
            //set(key_value[0].to_owned(), key_value[1].to_owned(), &store);
            eprintln!("unimplemented");
            process::exit(1)
        }

        Opt::Get {
            key,
            all,
            interactive,
        } => {
            //get(key, &store);
            eprintln!("unimplemented");
            process::exit(1)
        }
        Opt::Rm {
            key,
            all,
            interactive,
        } => {
            //rm(key, &store);
            eprintln!("unimplemented");
            process::exit(1)
        }
    }
}

fn set(key: String, value: String, store: &KvStore) {
    unimplemented!();
    store.set(key, value);
}

fn get(key: String, store: &KvStore) {
    unimplemented!();
    let res = store.get(key.to_owned());
    match res {
        Some(val) => println!("{}", val),
        _ => eprintln!("No value for this key , {}", key),
    };
}

fn rm(key: String, store: &KvStore) {
    unimplemented!();
    let res = store.get(key.to_owned());
    match res {
        Some(val) => println!("{}", val),
        _ => eprintln!("No value for this key , {}", key),
    };
}
