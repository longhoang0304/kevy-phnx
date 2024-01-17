use std::collections::VecDeque;
use std::error::Error;

use crate::command_processors::parsers::cores::{CommandParseError, CommandParser};
use crate::exe_engine::cores::{Command, CommandParameter};

pub struct Get;

impl CommandParser for Get {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        if tokens.is_empty() {
            let err = Box::new(CommandParseError::WrongNumberOfArguments(1, 0));
            return Err(err);
        }

        let key = tokens[0].clone();
        let parameters = VecDeque::from([CommandParameter::String(key)]);

        Ok(Command::new(
            Get::name(),
            Some(parameters),
        ))
    }

    fn name() -> &'static str {
        "GET"
    }
}
