use std::error::Error;
use std::iter::zip;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::{Storage, StorageData};

pub struct MSet;

impl CommandExecutor for MSet {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let keys: Vec<String> = cmd.get_required("KEYS")?;
        let values: Vec<String> = cmd.get_required("VALUES")?;

        let kv = zip(keys, values);
        for (key, value) in kv {
            let mut entry = storage.read(&key)?;

            entry.set_data(StorageData::from(value));
            storage.write(entry)?;
        }

        Ok(CommandResult::from(true))
    }
}
