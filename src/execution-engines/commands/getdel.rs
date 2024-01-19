use std::error::Error;

use crate::exe_engine::commands::funcs::get_required;
use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct GetDel;

impl CommandExecutor for GetDel {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = get_required("KEY", cmd)?;
        let entry = storage.read(&key)?;

        storage.delete(&key)?;

        Ok(CommandResult::from(entry.value.data))
    }
}
