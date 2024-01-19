use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::{Storage, StorageData};

pub struct SetRange;

impl CommandExecutor for SetRange {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let offset: usize = cmd.get_required("OFFSET")?;
        let value: String = cmd.get_required("VALUE")?;

        let mut entry = storage.read(&key)?;

        if entry.is_nil() {
            return Ok(CommandResult::Nil);
        }

        if !entry.is_primitive() {
            let err = Box::new(CommandExecutorError::NotSupportedDataType);
            return Err(err);
        }

        let mut data = entry.get_data().to_string();

        if offset > data.len() {
            let err = Box::new(CommandExecutorError::InvalidArgumentValue(String::from("OFFSET")));
            return Err(err);
        }

        data.insert_str(offset, value.as_str());
        let new_len = data.len() as i128;

        entry.set_data(StorageData::from(data));
        storage.write(entry)?;

        Ok(CommandResult::from(new_len))
    }
}
