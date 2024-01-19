use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct Unknown;

impl CommandExecutor for Unknown {
    fn execute(_: &mut Box<dyn Storage>, _: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let err = Box::new(CommandExecutorError::UnknownCommand);
        Err(err)
    }
}
