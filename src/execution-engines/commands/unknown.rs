use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutorError, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct Unknown;

impl CommandExecutor for Unknown {
    fn execute(_: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let err = Box::new(CommandExecutorError::UnknownCommand(String::from(cmd.name)));
        Err(err)
    }
}
