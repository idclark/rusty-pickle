# Rusty Pickle: Learning :crab: while dealing with :crab:

`Rusty-Pickle` is my personal project to learn Rust while I had/have some "downtime". 
It's based almost verbatim on a Python library called [Pickledb](https://pypi.org/project/pickleDB/).
It's an in-mem key value store with an option to serialize to json.
It'll eventually include parquet serialization. 

## Example Usage 
Refer to the [Examples/](https://github.com/idclark/rusty-pickle/tree/main/examples) directory:

``` rust
use rusty_pickle::{DumpPolicy, Pickle, SerializationMethod};

fn main() {
    let mut db = Pickle::new("test.db", DumpPolicy::Auto, SerializationMethod::Json);

    // set the value 100 to the key 'key1'
    db.set("key1", &100).unwrap();

    // set the value 1.1 to the key 'key2'
    db.set("key2", &1.1).unwrap();

    // set the value 'hello world' to the key 'key3'
    db.set("key3", &String::from("hello world")).unwrap();

    let num_keys = db.key_count();
    println!("You have inserted {} keys", num_keys);

```
