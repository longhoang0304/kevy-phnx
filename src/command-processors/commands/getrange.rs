use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct GetRange;

impl CommandParser for GetRange {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 3 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(3, tk_len));
            return Err(err);
        }

        let mut token_iter = tokens.iter();
        let key = CommandArgumentValue::from(token_iter.next().unwrap().clone());
        let mut arguments = HashMap::from([
            ("KEY", key),
        ]);

        let start = token_iter.next().map(|e| e.parse::<i128>()).unwrap();
        let end = token_iter.next().map(|e| e.parse::<i128>()).unwrap();

        if start.is_err() {
            let err = Box::new(CommandParserError::InvalidArgumentValue(String::from("START")));
            return Err(err);
        }
        if end.is_err() {
            let err = Box::new(CommandParserError::InvalidArgumentValue(String::from("END")));
            return Err(err);
        }

        arguments.insert("START", CommandArgumentValue::from(start.unwrap()));
        arguments.insert("END", CommandArgumentValue::from(end.unwrap()));

        Ok(Command::new(
            GetRange::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "GETRANGE"
    }
}
