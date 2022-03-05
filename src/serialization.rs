use core::fmt;

use std::collections::HashMap;

use serde::{de::DeserializeOwned, Serialize};

type DbMap = HashMap<String, Vec<u8>>;
type DbMapList = HashMap<String, Vec<Vec<u8>>>;

// Currently we will start with json serialization, because I unserstand it. Binary and maybe yaml coming later. maybe parquet

#[derive(Debug)]
pub enum SerializationMethod {
    Json,
    // more stuff to come...
}

impl From<i32> for SerializationMethod {
    fn from(item: i32) -> Self {
        match item {
            0 => SerializationMethod::Json,
            _ => SerializationMethod::Json,
        }
    }
}

impl fmt::Display for SerializationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct JsonSerializer {}

impl JsonSerializer {
    fn new() -> JsonSerializer {
        JsonSerializer {}
    }

    fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match serde_json::from_str(std::str::from_utf8(ser_data).unwrap()) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }

    fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match serde_json::to_string(data) {
            Ok(ser_data) => Ok(ser_data.into_bytes()),
            Err(err) => Err(err.to_string()),
        }
    }
}

// crate a struct to hold all of our serialization methods. Right now that's just json.
pub(crate) struct Serializer {
    ser_method: SerializationMethod,
    json_serializer: JsonSerializer,
}

impl Serializer {
    pub(crate) fn new(ser_method: SerializationMethod) -> Serializer {
        Serializer {
            ser_method,
            json_serializer: JsonSerializer::new(),
            // deserialize_data
            // serialize_data
        }
    }

    pub(crate) fn deserialize_data<V>(&self, ser_data: &[u8]) -> Option<V>
    where
        V: DeserializeOwned,
    {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.deserialize_data(ser_data),
        }
    }

    pub(crate) fn serialize_data<V>(&self, data: &V) -> Result<Vec<u8>, String>
    where
        V: Serialize,
    {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.serialize_data(data),
        }
    }
    // we need these for the dumpdb calls in rustypickle.rs
    pub(crate) fn serialize_db(
        &self,
        map: &DbMap,
        list_map: &DbMapList,
    ) -> Result<Vec<u8>, String> {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.serialize_db(map, list_map),
        }
    }

    pub(crate) fn deserialize_db(&self, ser_db: &[u8]) -> Result<(DbMap, DbMapList), String> {
        match self.ser_method {
            SerializationMethod::Json => self.json_serializer.deserialize_db(ser_db),
        }
    }
}
