use std::collections::VecDeque;
use std::error::Error;

use crate::exe_engine::cores::{Command, CommandParameter};
use crate::command_processors::parsers::cores::{CommandParser, CommandParseError};

pub struct Get;

impl CommandParser for Get {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        if tokens.is_empty() {
            let err = Box::new(CommandParseError::MissingParametersForCommand("GET"));
            return Err(err);
        }

        let key = tokens[0].clone();
        let parameters = VecDeque::from([CommandParameter::String(key)]);

        Ok(Command::new(
            "GET",
            Some(parameters),
        ))
    }
}
