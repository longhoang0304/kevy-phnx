use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum StorageData {
    Range(Vec<String>),
    Number(i128),
    String(String),
    Map(HashMap<String, String>),
    Nil,
}

impl Display for StorageData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            StorageData::Number(v) => v.to_string(),
            StorageData::String(v) => v.clone(),
            StorageData::Range(_) => String::from("[List]"),
            StorageData::Map(_) => String::from("[Map]"),
            StorageData::Nil => String::from("(nil)"),
        };

        write!(f, "{val}")
    }
}

impl From<String> for StorageData {
    fn from(value: String) -> Self {
        StorageData::String(value)
    }
}

impl From<i128> for StorageData {
    fn from(value: i128) -> Self {
        StorageData::Number(value)
    }
}

impl StorageData {
    pub fn is_primitive(&self) -> bool {
        match self {
            StorageData::Number(_) | StorageData::String(_) | StorageData::Nil => true,
            _ => false,
        }
    }
}