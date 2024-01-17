use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct Ping;

impl CommandExecutor for Ping {
    fn execute(_: &mut Box<dyn Storage>, _: &Command) -> Result<CommandResult, Box<dyn Error>> {
        Ok(CommandResult::from(String::from("Pong")))
    }
}
