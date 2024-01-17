use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutorError, CommandExecutor, CommandParameter, CommandResult};
use crate::storage::cores::{Storage, StorageData, StorageEntry, StorageValue};

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
                let err = Box::new(CommandExecutorError::InvalidKey);
                return Err(err);
            }
        };

        let value = StorageValue::new(StorageData::from(value), None);
        let entry = StorageEntry::new(key, value);

        storage.write(entry)?;
        Ok(CommandResult::Bool(true))
    }
}
