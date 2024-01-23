use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct DecrBy;

impl CommandParser for DecrBy {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let key = CommandArgumentValue::from(tokens[0].clone());
        let amount = tokens[1].parse::<i128>();

        if amount.is_err() {
            let err = Box::new(CommandParserError::InvalidArgumentValue(String::from("amount")));
            return Err(err);
        }

        let amount = CommandArgumentValue::from(amount.unwrap());
        let arguments = HashMap::from([
            ("KEY", key),
            ("AMOUNT", amount),
        ]);

        Ok(Command::new(
            DecrBy::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "DECRBY"
    }
}
