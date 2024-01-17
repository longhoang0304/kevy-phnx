use std::collections::VecDeque;

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::Command;

pub struct Unknown;

impl CommandParser for Unknown {
    fn parse(_: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        Ok(Command::new(
            Unknown::name(),
            None,
        ))
    }

    fn name() -> &'static str {
        "UNKNOWN"
    }
}
