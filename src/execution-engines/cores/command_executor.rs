use std::error::Error;

use crate::storage::cores::Storage;

use super::{Command, CommandResult};

pub trait CommandExecutor {
    fn execute(storage: &mut Box<dyn Storage>, command: &Command) -> Result<CommandResult, Box<dyn Error>>;
}
