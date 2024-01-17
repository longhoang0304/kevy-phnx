use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::{Storage, StorageData, StorageEntry, StorageValue};

pub struct Set;

impl CommandExecutor for Set {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let parameters = cmd.parameters.as_ref().unwrap();
        let mut params_iter = parameters.iter();
        let key: String = params_iter.next().unwrap().clone().try_into()?;
        let value: String = params_iter.next().unwrap().clone().try_into()?;

        let value = StorageValue::new(StorageData::from(value), None);
        let entry = StorageEntry::new(key, value);

        storage.write(entry)?;
        Ok(CommandResult::from(true))
    }
}
