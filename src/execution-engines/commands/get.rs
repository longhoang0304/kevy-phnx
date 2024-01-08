use std::error::Error;

use crate::exe_engine::cores::{Command, CommandError, CommandExecutor, CommandParameter, CommandResult};
use crate::storage::cores::Storage;

pub struct Get;

impl CommandExecutor for Get {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let parameters = cmd.parameters.as_ref().unwrap();
        let key_param = parameters.front().unwrap();
        let key;

        if let CommandParameter::String(v) = key_param {
            key = v.clone();
        } else {
            let err = Box::new(CommandError::InvalidKey);
            return Err(err);
        }

        if key.is_empty() {
            let err = Box::new(CommandError::MissingKey);
            return Err(err);
        }

        let entry = storage.read(&key)?;
        if entry.is_none() {
            let err = Box::new(CommandError::ValueNotFound(key));
            return Err(err);
        }

        Ok(CommandResult::from(entry.unwrap().value))
    }
}
