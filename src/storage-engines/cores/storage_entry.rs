use std::collections::HashMap;

pub type StorageKey = String;

#[derive(Debug, Clone)]
pub enum StorageValue {
    Range(Vec<String>),
    Number(i64),
    String(String),
    Map(HashMap<String, String>),
}

pub struct StorageEntry {
    pub key: StorageKey,
    pub value: StorageValue,
}

impl StorageEntry {
    pub fn new(key: StorageKey, value: StorageValue) -> StorageEntry {
        StorageEntry {
            key,
            value,
        }
    }
}

pub type StorageEntries = HashMap<StorageKey, StorageValue>;
