use std::error::Error;

use crate::exe_engine::commands::funcs::get_required;
use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::{Storage, StorageData, StorageEntry, StorageValue};

pub struct Set;

impl CommandExecutor for Set {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = get_required("KEY", cmd)?;
        let value: String = get_required("VALUE", cmd)?;

        let value = StorageValue::new(StorageData::from(value), None);
        let entry = StorageEntry::new(key, value);

        storage.write(entry)?;
        Ok(CommandResult::from(true))
    }
}
