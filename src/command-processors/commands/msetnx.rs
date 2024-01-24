use std::collections::VecDeque;

use crate::command_processors::commands::MSet;
use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::Command;

pub struct MSetNx;

impl CommandParser for MSetNx {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let command = MSet::parse(tokens)?;
        let arguments = command.arguments;

        Ok(Command::new(
            Self::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "MSETNX"
    }
}
