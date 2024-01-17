use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use crate::storage::cores::StorageValue;

#[derive(Debug)]
pub enum CommandResult {
    Range(Vec<String>),
    Number(i64),
    String(String),
    Map(HashMap<String, String>),
    Bool(bool),
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
            _ => String::from(""),
        };

        write!(f, "{msg}")
    }
}

impl From<StorageValue> for CommandResult {
    fn from(value: StorageValue) -> Self {
        match value {
            StorageValue::Range(val) => CommandResult::Range(val),
            StorageValue::Number(val) => CommandResult::Number(val),
            StorageValue::String(val) => CommandResult::String(val),
            StorageValue::Map(val) => CommandResult::Map(val),
        }
    }
}

impl CommandResult {}

