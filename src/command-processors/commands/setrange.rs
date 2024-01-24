use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct SetRange;

impl CommandParser for SetRange {
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

        let offset = token_iter.next().map(|e| e.parse::<usize>()).unwrap();
        let value = token_iter.next().unwrap().clone();

        if offset.is_err() {
            let err = Box::new(CommandParserError::InvalidArgumentValue(String::from("OFFSET")));
            return Err(err);
        }

        arguments.insert("OFFSET", CommandArgumentValue::from(offset.unwrap()));
        arguments.insert("VALUE", CommandArgumentValue::from(value));

        Ok(Command::new(
            SetRange::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "SETRANGE"
    }
}
