use std::collections::HashMap;

use super::{CommandArgumentKey, CommandArgumentValue};

pub type CommandArguments = HashMap<CommandArgumentKey, CommandArgumentValue>;

#[derive(Debug)]
pub struct Command {
    pub name: &'static str,
    pub arguments: CommandArguments,
    pub database: u8,
}

impl Command {
    pub fn new(name: &'static str, arguments: CommandArguments) -> Command {
        Command {
            name,
            arguments,
            database: 0,
        }
    }
}
