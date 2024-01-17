use std::collections::VecDeque;
use std::error::Error;

use crate::command_processors::parsers::cores::CommandParser;
use crate::exe_engine::cores::Command;

pub struct Ping;

impl CommandParser for Ping {
    fn parse(_: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        Ok(Command::new(Ping::name(), None))
    }

    fn name() -> &'static str {
        "PING"
    }
}
