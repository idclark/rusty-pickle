pub use rusty_pickle::Pickle;

#[cfg(test)]
mod tests {
    use rusty_pickle::DumpPolicy;

    use super::*;

    #[test]
    fn test_set_get() {
        let mut db = Pickle::new(
            "test_name",
            DumpPolicy::Auto,
            rusty_pickle::SerializationMethod::Json,
        );

        let num = 100;
        let string = String::from("hello");
        db.set("num", &num).unwrap();
        db.set("a string", &string).unwrap();
        // read a num
        assert_eq!(db.get::<i32>("num").unwrap(), num);
        // read a string
        assert_eq!(db.get::<String>("a string").unwrap(), "hello");
    }

    #[test]
    fn test_dump_db() {
        let mut db = Pickle::new(
            "test_name",
            DumpPolicy::UponRequest,
            rusty_pickle::SerializationMethod::Json,
        );
        let num = 100;
        let string = String::from("hello");
        db.set("num", &num).unwrap();
        db.set("a string", &string).unwrap();
        assert!(db.dump().is_ok())
    }

    #[test]
    fn test_load_db() {
        let mut db = Pickle::new(
            "test.db",
            DumpPolicy::Auto,
            rusty_pickle::SerializationMethod::Json,
        );
        assert!(db.load("test.db").is_ok())
    }

    #[test]
    fn test_get_all_keys() {
        let mut db = Pickle::new(
            "test.db",
            DumpPolicy::Auto,
            rusty_pickle::SerializationMethod::Json,
        );

        let key_count = 10;
        let dummy_value = 1;

        for i in 0..10 {
            db.set(&format!("{}{}", "key", i), &dummy_value).unwrap();
        }

        assert_eq!(db.key_count(), key_count);
    }

    #[test]
    fn test_list_keys() {
        let mut db = Pickle::new(
            "test.db",
            DumpPolicy::Auto,
            rusty_pickle::SerializationMethod::Json,
        );
    }
}
