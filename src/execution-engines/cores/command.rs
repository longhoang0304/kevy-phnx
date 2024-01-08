use std::collections::VecDeque;

use super::CommandParameter;

#[derive(Debug)]
pub struct Command {
    pub name: &'static str,
    pub parameters: Option<VecDeque<CommandParameter>>,
    pub database: u8,
}

impl Command {
    pub fn new(name: &'static str, parameters: Option<VecDeque<CommandParameter>>) -> Command {
        Command {
            name,
            parameters,
            database: 0,
        }
    }
}
