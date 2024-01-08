use std::error::Error;

use super::{Command, CommandResult};

pub trait ExecuteEngine {
    fn execute(&mut self, command: Command) -> Result<CommandResult, Box<dyn Error>>;
}
