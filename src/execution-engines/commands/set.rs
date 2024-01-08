use std::error::Error;

use crate::exe_engine::cores::{Command, CommandError, CommandExecutor, CommandParameter, CommandResult};
use crate::storage::cores::{Storage, StorageEntry, StorageValue};

pub struct Set;

impl CommandExecutor for Set {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let parameters = cmd.parameters.as_ref().unwrap();
        let key_param = parameters.front().unwrap();
        let value_param = parameters.back().unwrap();
        let key;
        let value;

        match (key_param, value_param) {
            (CommandParameter::String(k), CommandParameter::String(v)) => {
                key = k.clone();
                value = v.clone();
            }
            _ => {
                let err = Box::new(CommandError::InvalidKey);
                return Err(err);
            }
        };

        if key.is_empty() {
            let err = Box::new(CommandError::MissingKey);
            return Err(err);
        }

        let entry = StorageEntry::new(key, StorageValue::String(value));

        storage.write(entry)?;
        Ok(CommandResult::Bool(true))
    }
}
