use std::error::Error;

use super::SetEx;
use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct GetEx;

impl CommandExecutor for GetEx {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let entry = storage.read(&key)?;
        let data = entry.get_data().clone();

        SetEx::execute(storage, cmd)?;
        Ok(CommandResult::from(data))
    }
}
