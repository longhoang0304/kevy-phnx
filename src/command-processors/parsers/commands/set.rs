use std::collections::VecDeque;
use std::error::Error;

use crate::exe_engine::cores::{Command, CommandParameter};
use crate::command_processors::parsers::cores::{CommandParser, CommandParseError};

pub struct Set;

impl CommandParser for Set {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        if tokens.len() != 2 {
            let err = Box::new(CommandParseError::MissingParametersForCommand("SET"));
            return Err(err);
        }

        let key = tokens.front().unwrap().clone();
        let value = tokens.back().unwrap().clone();
        let parameters = VecDeque::from([
            CommandParameter::String(key),
            CommandParameter::String(value),
        ]);

        Ok(Command::new(
            "SET",
            Some(parameters),
        ))
    }
}
