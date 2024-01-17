use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::storage::cores::StorageData;

#[derive(Debug)]
pub enum CommandResult {
    Range(Vec<String>),
    Number(i128),
    String(String),
    Map(HashMap<String, String>),
    Bool(bool),
    Nil,
}

impl Display for CommandResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandResult::Number(v) => v.to_string(),
            CommandResult::String(v) => v.clone(),
            CommandResult::Bool(v) => {
                if *v == true {
                    String::from("OK")
                } else {
                    String::from("NOT OK - CHECK ERROR MESSAGE")
                }
            }
            CommandResult::Nil => String::from("(nil)"),
            _ => String::from(""),
        };

        write!(f, "{msg}")
    }
}

impl From<StorageData> for CommandResult {
    fn from(value: StorageData) -> Self {
        match value {
            StorageData::Range(val) => CommandResult::Range(val),
            StorageData::Number(val) => CommandResult::Number(val),
            StorageData::String(val) => CommandResult::String(val),
            StorageData::Map(val) => CommandResult::Map(val),
            StorageData::Nil => CommandResult::Nil,
        }
    }
}

impl From<&StorageData> for CommandResult {
    fn from(value: &StorageData) -> Self {
        match value.clone() {
            StorageData::Range(val) => CommandResult::Range(val),
            StorageData::Number(val) => CommandResult::Number(val),
            StorageData::String(val) => CommandResult::String(val),
            StorageData::Map(val) => CommandResult::Map(val),
            StorageData::Nil => CommandResult::Nil,
        }
    }
}

impl From<String> for CommandResult {
    fn from(value: String) -> Self {
        CommandResult::String(value)
    }
}

impl From<i128> for CommandResult {
    fn from(value: i128) -> Self {
        CommandResult::Number(value)
    }
}

impl From<&str> for CommandResult {
    fn from(value: &str) -> Self {
        CommandResult::String(String::from(value))
    }
}



impl CommandResult {}

