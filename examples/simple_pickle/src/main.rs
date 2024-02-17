//! A simple example of how to use the Pickle database. It includes:
//! * Creating a new DB
//! * Loading an existing DB from a file
//! * Setting and getting key-value pairs of different types

use std::fmt::{self, Display, Formatter};

use rusty_pickle::{DumpPolicy, Pickle, SerializationMethod};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Rectangle {
    width: i32,
    length: i32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Rectangle: length={}, width={}", self.length, self.width)
    }
}

fn main() {
    // create a new DB with Autodump so every change is written to file
    let mut db = Pickle::new("example.db", DumpPolicy::Auto, SerializationMethod::Json);

    db.set("key1", &100).unwrap();
    db.set("key2", &1.1).unwrap();
    db.set("key3", &String::from("hello world")).unwrap();
    db.set("key4", &vec![1, 2, 3]).unwrap();

    db.set(
        "key5",
        &Rectangle {
            width: 4,
            length: 10,
        },
    )
    .unwrap();

    // load an existing DB from a file (the same file in this case)
    let db2 = Pickle::load(
        "/Users/ian/rusty-pickle/example.db",
        DumpPolicy::UponRequest,
        SerializationMethod::Json,
    )
    .unwrap();

    // print the value of key1
    println!(
        "The value of key1 is: {}",
        db2.get::<String>("key1").unwrap()
    );

    println!(
        "Value of key1 as loaded from file is: {}",
        db2.get::<String>("key1").unwrap()
    );
}
