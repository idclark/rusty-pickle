use serde::{de::DeserializedOwned, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub enum DumpPolicy {
    Never,
    Auto,
    UponRequest,
    Periodic(Duration),
}

pub struct Pickle {
    map: HashMap<String, Vec<u8>>,
    list_map: HashMap<String, Vec<Vec<u8>>>,
    db_file_path: PathBuf,
    dump_policy: DumpPolicy,
    // last dump
}

impl Pickle {
    pub fn new<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Pickle {
        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        Pickle {
            map: HashMap::new(),
            list_map: HashMap::new(),
            db_file_path: db_path_buf,
            dump_policy,
            // last dump TODO
        }
    }

    pub fn get<V>(&self, key: &str) -> Option<V>
    where
        V: DeserializedOwned,
    {
        match self.map.get(key) {
            Some(val) => self.serializer.deserialize_data::<V>(&val),
            None => None,
        }
    }

    pub fn set<V>(&mut self, key: &str, value: &V) -> Result<()>
    where
        V: Serialize,
    {
        if self.list_map.contains_key(key) {
            self.list_map.remove(key);
        }
        let ser_data = match self.serializer.serialize_data(value) {
            Ok(data) => data,
            Err(err_str) => return Err(Error::new(ErrorCode::Serialization(err_str))),
        };

        let original_value = self.map.insert(String::from(key), ser_data);
        match self.dumpdb() {
            Ok(_) => Ok(()),
            Err(err) => {
                match original_value {
                    None => {
                        self.map.remove(key);
                    }
                    Some(orig_value) => {
                        self.map.insert(String::from(key), orig_value.to_vec());
                    }
                }

                Err(err)
            }
        }
    }

    /// Check if a key exists.
    ///
    /// This method returns `true` if the key exists and `false` otherwise.
    ///
    /// # Arguments
    ///
    /// * `key` - the key to check
    ///
    pub fn exists(&self, key: &str) -> bool {
        self.map.get(key).is_some() || self.list_map.get(key).is_some()
    }
}

   

