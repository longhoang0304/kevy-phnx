use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct GetEx;

impl CommandParser for GetEx {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tokens.is_empty() {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(1, 0));
            return Err(err);
        }

        let mut token_iter = tokens.iter();
        let key = CommandArgumentValue::from(token_iter.next().unwrap().clone());
        let mut arguments = HashMap::from([
            ("KEY", key),
        ]);

        let time_unit = token_iter.next().map(|e| e.to_uppercase());
        if time_unit.is_none() {
            return Ok(Command::new(
                GetEx::name(),
                arguments,
            ));
        }

        // ========

        let time_unit = time_unit.unwrap().clone();

        if tk_len == 2 && time_unit.as_str() != "PERSIST" {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        if tk_len == 2 {
            arguments.insert("TTL_UNIT", CommandArgumentValue::from(time_unit));
            return Ok(Command::new(
                GetEx::name(),
                arguments,
            ));
        }

        // ========

        if tk_len > 3 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(3, tk_len));
            return Err(err);
        }

        let valid_unit = match time_unit.as_str() {
            "EX" | "PX" | "EXAT" | "PXAT" => true,
            _ => false,
        };

        if !valid_unit {
            let err = Box::new(CommandParserError::InvalidArgument(time_unit));
            return Err(err);
        }

        let time_value = token_iter.next().map(|e| e.parse::<i128>()).unwrap();
        if let Err(_) = time_value {
            let err = Box::new(CommandParserError::InvalidArgumentValue(time_unit));
            return Err(err);
        }

        let time_value = time_value.unwrap();

        arguments.insert("TTL_UNIT", CommandArgumentValue::from(time_unit));
        arguments.insert("TTL_VALUE", CommandArgumentValue::from(time_value));

        // ========

        Ok(Command::new(
            GetEx::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "GETEX"
    }
}
