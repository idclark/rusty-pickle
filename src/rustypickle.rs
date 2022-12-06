use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::fs;
use std::iter::Inspect;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::error::{Error, ErrorCode, Result};
use crate::serialization::SerializationMethod;
use crate::serialization::Serializer;

pub enum DumpPolicy {
    Never,
    Auto,
    UponRequest,
    Periodic(Duration),
}

pub struct Pickle {
    map: HashMap<String, Vec<u8>>,
    list_map: HashMap<String, Vec<Vec<u8>>>,
    serializer: Serializer,
    db_file_path: PathBuf,
    dump_policy: DumpPolicy,
    last_dump: Instant,
}

impl Pickle {
    pub fn new<P: AsRef<Path>>(
        db_path: P,
        dump_policy: DumpPolicy,
        serialization_method: SerializationMethod,
    ) -> Pickle {
        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        Pickle {
            map: HashMap::new(),
            list_map: HashMap::new(),
            serializer: Serializer::new(serialization_method),
            db_file_path: db_path_buf,
            dump_policy,
            last_dump: Instant::now(),
        }
    }

    pub fn get<V>(&self, key: &str) -> Option<V>
    where
        V: DeserializeOwned,
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

    pub fn dump(&mut self) -> Result<()> {
        if let DumpPolicy::Never = self.dump_policy {
            return Ok(());
        }

        match self.serializer.serialize_db(&self.map, &self.list_map) {
            Ok(ser_db) => {
                let temp_file_path = format!(
                    "{}.temp.{}",
                    self.db_file_path.to_str().unwrap(),
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                );

                match fs::write(&temp_file_path, ser_db) {
                    Ok(_) => (),
                    Err(err) => return Err(Error::new(ErrorCode::Io(err))),
                }

                match fs::rename(temp_file_path, &self.db_file_path) {
                    Ok(_) => (),
                    Err(err) => return Err(Error::new(ErrorCode::Io(err))),
                }

                if let DumpPolicy::Periodic(_dur) = self.dump_policy {
                    self.last_dump = Instant::now();
                }
                Ok(())
            }
            Err(err_str) => Err(Error::new(ErrorCode::Serialization(err_str))),
        }
    } // end dump method

    fn dumpdb(&mut self) -> Result<()> {
        match self.dump_policy {
            DumpPolicy::Auto => self.dump(),
            DumpPolicy::Periodic(duration) => {
                let now = Instant::now();
                if now.duration_since(self.last_dump) > duration {
                    self.last_dump = Instant::now();
                    self.dump()?;
                }
                Ok(())
            }
            _ => Ok(()),
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

    pub fn key_count(&self) -> usize {
        // the latter addition is moot until the methods are added
        self.map.iter().len() + self.list_map.iter().len()
    }
}
