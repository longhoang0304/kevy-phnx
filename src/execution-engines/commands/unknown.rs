use std::error::Error;

use crate::exe_engine::cores::{Command, CommandError, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct Unknown;

impl CommandExecutor for Unknown {
    fn execute(_: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let err = Box::new(CommandError::UnknownCommand(String::from(cmd.name)));
        Err(err)
    }
}
