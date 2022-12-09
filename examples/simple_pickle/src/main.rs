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

    let keys = db.list_keys();
    println!("Now we will print all of the keys in the database");
    keys.iter().for_each(|k| println!("{}", k));

    println!("Now we will load the json file into a new database and get the values");
    let db2 = Pickle::load(
        "test.db",
        DumpPolicy::UponRequest,
        SerializationMethod::Json,
    )
    .unwrap();

    println!("Getting the value for key: key1");
    let val1: String = db2.get::<String>("key1").unwrap();
    println!("The value is {}", val1);
}
