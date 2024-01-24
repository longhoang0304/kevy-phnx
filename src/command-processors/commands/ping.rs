use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::Command;

pub struct Ping;

impl CommandParser for Ping {
    fn parse(_: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        Ok(Command::new(Ping::name(), HashMap::new()))
    }

    fn name() -> &'static str {
        "PING"
    }
}
