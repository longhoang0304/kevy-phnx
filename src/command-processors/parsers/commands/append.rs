use std::collections::VecDeque;

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandParameter};

pub struct Append;

impl CommandParser for Append {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let key = tokens.front().unwrap().clone();
        let value = tokens.back().unwrap().clone();
        let parameters = VecDeque::from([CommandParameter::String(key), CommandParameter::String(value)]);

        Ok(Command::new(
            Append::name(),
            Some(parameters),
        ))
    }

    fn name() -> &'static str {
        "APPEND"
    }
}
