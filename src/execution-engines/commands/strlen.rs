use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct Strlen;

impl CommandExecutor for Strlen {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let entry = storage.read(&key)?;
        let data = entry.get_data().to_string();

        if !entry.is_primitive() {
            let err = Box::new(CommandExecutorError::NotSupportedDataType);
            return Err(err);
        }

        if entry.is_nil() {
            return Ok(CommandResult::from(0));
        }

        Ok(CommandResult::from(data.len() as i128))
    }
}
