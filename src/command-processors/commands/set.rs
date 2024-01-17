use std::collections::VecDeque;

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandParameter};

pub struct Set;

impl CommandParser for Set {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let key = tokens.front().unwrap();
        let value = tokens.back().unwrap();
        let parameters = VecDeque::from([
            CommandParameter::from(key),
            CommandParameter::from(value),
        ]);

        Ok(Command::new(
            Set::name(),
            Some(parameters),
        ))
    }

    fn name() -> &'static str {
        "SET"
    }
}
