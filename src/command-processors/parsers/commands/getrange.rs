use std::collections::VecDeque;
use std::error::Error;

use crate::command_processors::parsers::cores::{CommandParseError, CommandParser};
use crate::exe_engine::cores::{Command, CommandParameter};

pub struct GetRange;

impl CommandParser for GetRange {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 3 {
            let err = Box::new(CommandParseError::WrongNumberOfArguments(3, tk_len));
            return Err(err);
        }

        let mut token_iter = tokens.iter();
        let key = token_iter.next().unwrap().clone();
        let mut parameters = VecDeque::from([CommandParameter::String(key)]);

        let start = token_iter.next().map(|e| e.parse::<i64>()).unwrap();
        let end = token_iter.next().map(|e| e.parse::<i64>()).unwrap();

        if start.is_err() {
            let err = Box::new(CommandParseError::InvalidArgumentValue(String::from("start")));
            return Err(err);
        }
        if end.is_err() {
            let err = Box::new(CommandParseError::InvalidArgumentValue(String::from("end")));
            return Err(err);
        }

        parameters.push_back(CommandParameter::Number(start.unwrap()));
        parameters.push_back(CommandParameter::Number(end.unwrap()));

        Ok(Command::new(
            GetRange::name(),
            Some(parameters),
        ))
    }

    fn name() -> &'static str {
        "GETRANGE"
    }
}
