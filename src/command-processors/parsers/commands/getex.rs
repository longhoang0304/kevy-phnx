use std::collections::VecDeque;
use std::error::Error;

use crate::command_processors::parsers::cores::{CommandParseError, CommandParser};
use crate::exe_engine::cores::{Command, CommandParameter};

pub struct GetEx;

impl CommandParser for GetEx {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>> {
        let tk_len = tokens.len() as u16;
        if tokens.is_empty() {
            let err = Box::new(CommandParseError::WrongNumberOfArguments(1, 0));
            return Err(err);
        }

        let mut token_iter = tokens.iter();
        let key = token_iter.next().unwrap().clone();
        let mut parameters = VecDeque::from([CommandParameter::String(key)]);

        let time_unit = token_iter.next().map(|e| e.to_uppercase());
        if time_unit.is_none() {
            return Ok(Command::new(
                GetEx::name(),
                Some(parameters),
            ));
        }

        // ========

        let time_unit = time_unit.unwrap().clone();

        if tk_len == 2 && time_unit.as_str() != "PERSIST" {
            let err = Box::new(CommandParseError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        if tk_len == 2 {
            parameters.push_back(CommandParameter::String(time_unit));
            return Ok(Command::new(
                GetEx::name(),
                Some(parameters),
            ));
        }

        // ========

        if tk_len > 3 {
            let err = Box::new(CommandParseError::WrongNumberOfArguments(3, tk_len));
            return Err(err);
        }

        let valid_unit = match time_unit.as_str() {
            "EX" | "PX" | "EXAT" | "PXAT" => true,
            _ => false,
        };

        if !valid_unit {
            let err = Box::new(CommandParseError::InvalidArgument(time_unit));
            return Err(err);
        }

        let time_value = token_iter.next().map(|e| e.parse::<u32>()).unwrap();
        if let Err(_) = time_value {
            let err = Box::new(CommandParseError::InvalidArgumentValue(time_unit));
            return Err(err);
        }

        let time_value = Box::new(CommandParameter::Number(time_value.unwrap() as i64));

        parameters.push_back(CommandParameter::Pair(time_unit, time_value));

        // ========

        Ok(Command::new(
            GetEx::name(),
            Some(parameters),
        ))
    }

    fn name() -> &'static str {
        "GETEX"
    }
}
