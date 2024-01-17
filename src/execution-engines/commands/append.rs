use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::{Storage, StorageData};

pub struct Append;

impl CommandExecutor for Append {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let parameters = cmd.parameters.as_ref().unwrap();
        let mut params_iter = parameters.iter();
        let key: String = params_iter.next().unwrap().clone().try_into()?;
        let value: String = params_iter.next().unwrap().clone().try_into()?;

        let mut entry = storage.read(&key)?;
        let old_data = entry.get_data().to_string();
        let new_data = old_data + value.as_str();
        let new_len = new_data.len() as i128;

        entry.set_data(StorageData::from(new_data));
        storage.write(entry)?;

        Ok(CommandResult::from(new_len))
    }
}
