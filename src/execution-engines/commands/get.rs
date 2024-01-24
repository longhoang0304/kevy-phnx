use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct Get;

impl CommandExecutor for Get {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let entry = storage.read(&key)?;
        let data = entry.get_data();

        Ok(CommandResult::from(data))
    }
}
