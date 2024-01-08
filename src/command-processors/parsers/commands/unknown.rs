use std::collections::VecDeque;
use std::error::Error;

use crate::command_processors::parsers::cores::CommandParser;
use crate::exe_engine::cores::Command;

pub struct Unknown;

impl CommandParser for Unknown {
    fn parse(_: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        Ok(Command::new(
            "UNKNOWN",
            None,
        ))
    }
}
