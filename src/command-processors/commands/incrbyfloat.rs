use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct IncrByFloat;

impl CommandParser for IncrByFloat {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let key = CommandArgumentValue::from(tokens[0].clone());
        let amount = tokens[1].parse::<f64>();

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
            IncrByFloat::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "INCRBYFLOAT"
    }
}
