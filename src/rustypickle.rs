use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::fs;
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
    /// Constructs a new `Pickle` instance.
    ///
    /// # Arguments
    ///
    /// * `db_path` - a path where the DB will be stored
    /// * `dump_policy` - an enum value that determines the policy of dumping DB changes into the file. Please see
    ///    [Pickle::load()](#method.load) to understand the different policy options
    /// * `serialization_method` - the serialization method to use for storing the data to memory and file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rusty_pickle::{Pickle, DumpPolicy, SerializationMethod};
    ///
    /// let mut db = Pickle::new("example.db", DbDumpPolicy::AutoDump, SerializationMethod::Json);
    /// ```
    ///
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

    pub fn load<P: AsRef<Path>>(
        db_path: P,
        dump_policy: DumpPolicy,
        serialization_method: SerializationMethod,
    ) -> Result<Pickle> {
        let content = match fs::read(db_path.as_ref()) {
            Ok(file_content) => file_content,
            Err(err) => return Err(Error::new(ErrorCode::Io(err))),
        };

        let serializer = Serializer::new(serialization_method);

        let maps_from_file: (_, _) = match serializer.deserialize_db(&content) {
            Ok(maps) => maps,
            Err(err_str) => return Err(Error::new(ErrorCode::Serialization(err_str))),
        };

        let mut db_path_buf = PathBuf::new();
        db_path_buf.push(db_path);

        Ok(Pickle {
            map: maps_from_file.0,
            list_map: maps_from_file.1,
            serializer,
            db_file_path: db_path_buf,
            dump_policy,
            last_dump: Instant::now(),
        })
    }

    pub fn load_json<P: AsRef<Path>>(db_path: P, dump_policy: DumpPolicy) -> Result<Pickle> {
        Pickle::load(db_path, dump_policy, SerializationMethod::Json)
    }

    /// Retrieve a value for a specified key
    /// It's the user's responsibility to know the value type and give it while calling this method.
    /// If the key doesn't exist or if the type is wrong, `None` will be returned.
    /// Otherwise `Some(V)` will be returned.
    ///
    /// # Arguments
    ///
    /// * `key` - the key for which you'd like to retrieve a value
    ///
    /// # Examples
    ///
    /// // read a num
    /// let num = db.get::<i32>("key1").unwrap();
    ///
    pub fn get<V>(&self, key: &str) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match self.map.get(key) {
            Some(val) => self.serializer.deserialize_data::<V>(&val),
            None => None,
        }
    }

    /// Set a key and its respective value
    ///
    /// # Arguments
    ///
    /// * `key` - A string that a value would be associated with
    /// * `value` - A piece of data to be stored
    ///
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

    /// Return the count of keys in the database
    ///
    /// This method returns a `usize` of the number of keys in the database
    ///
    /// # Examples
    ///
    /// let count = db.key_count();
    ///
    pub fn key_count(&self) -> usize {
        // the latter addition is moot until the methods are added
        self.map.iter().len() + self.list_map.iter().len()
    }

    /// Return a vector of keys in the database
    ///
    ///
    /// # Examples
    ///
    /// let key_array = db.list_keys()
    ///
    pub fn list_keys(&self) -> Vec<String> {
        let mut key_array: Vec<String> = Vec::new();
        for k in self.map.keys() {
            key_array.push(k.to_string());
        }
        key_array
    }

    /// Remove a key-value pair or a list from the DB.
    ///
    /// This methods returns `Ok(true)` if the key was found in the DB or `Ok(false)` if it wasn't found.
    /// # Arguments
    ///
    /// * `key` the key or list name to remove
    ///
    pub fn remove(&mut self, key: &str) -> Result<bool> {
        let remove_map = match self.map.remove(key) {
            None => None,
            Some(val) => match self.dumpdb() {
                Ok(_) => Some(val),
                Err(err) => {
                    self.map.insert(String::from(key), val);
                    return Err(err);
                }
            },
        };

        let remove_list = match self.list_map.remove(key) {
            None => None,
            Some(list) => match self.dumpdb() {
                Ok(_) => Some(list),
                Err(err) => {
                    self.list_map.insert(String::from(key), list);
                    return Err(err);
                }
            },
        };

        Ok(remove_map.is_some() || remove_list.is_some())
    }
}
