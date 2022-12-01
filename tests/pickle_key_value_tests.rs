use rusty_pickle::Pickle;

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
        db.set("num", &num).unwrap();
        // read a num
        assert_eq!(db.get::<i32>("num").unwrap(), num);
    }
}
// #[test]
//     fn test_get() {}
// }
