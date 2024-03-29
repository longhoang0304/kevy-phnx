use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct Append;

impl CommandParser for Append {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let key = CommandArgumentValue::from(tokens.front().unwrap());
        let value = CommandArgumentValue::from(tokens.back().unwrap());
        let arguments = HashMap::from([
            ("KEY", key),
            ("VALUE", value),
        ]);

        Ok(Command::new(
            Append::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "APPEND"
    }
}
