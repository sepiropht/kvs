use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;
use std::fs::File;
use std::io::{Read, Write};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    x: i32,
    y: i32,
}

fn main() -> Result<(), Box<Error>> {
    let point = Move { x: 1, y: 2 };

    //   Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point)?;

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    fs::write("foo.txt", serialized)?;

    let mut file = File::open("foo.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Convert the JSON string back to a Point.
    let deserialized: Move = serde_json::from_str(&contents)?;

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
 
    Ok(())
}
