use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct Set;

impl CommandParser for Set {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len < 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let mut token_iter = tokens.iter();
        let key = CommandArgumentValue::from(token_iter.next().unwrap());
        let value = CommandArgumentValue::from(token_iter.next().unwrap());
        let mut arguments = HashMap::from([
            ("KEY", key),
            ("VALUE", value),
        ]);

        loop {
            let next = token_iter.next();
            if next.is_none() {
                break;
            }

            let next = next.map(|e| e.to_uppercase()).unwrap();
            let no_ttl = !arguments.contains_key("TTL_UNIT") && !arguments.contains_key("TTL_VALUE");
            let no_ttl_value = arguments.contains_key("TTL_UNIT") && !arguments.contains_key("TTL_VALUE");

            match next.as_str() {
                "GET" if !arguments.contains_key("RETURN_OLD_DATA") =>
                    arguments.insert("RETURN_OLD_DATA", CommandArgumentValue::from(true)),

                "NX" | "XX" if !arguments.contains_key("SET_CONDITION") =>
                    arguments.insert("SET_CONDITION", CommandArgumentValue::from(next)),

                "EX" | "PX" | "PXAT" | "EXAT" | "PERSIST" if no_ttl =>
                    arguments.insert("TTL_UNIT", CommandArgumentValue::from(next)),

                _ if no_ttl_value => {
                    let ttl_value = next.parse::<i128>();
                    if ttl_value.is_err() {
                        let err = Box::new(CommandParserError::InvalidArgumentValue(next));
                        return Err(err);
                    }

                    let ttl_value = ttl_value.unwrap();
                    if ttl_value < 1 {
                        let err = Box::new(CommandParserError::InvalidArgumentValue(next));
                        return Err(err);
                    }

                    arguments.insert("TTL_VALUE", CommandArgumentValue::from(ttl_value))
                }

                _ => {
                    let err = Box::new(CommandParserError::InvalidSyntax);
                    return Err(err);
                }
            };
        }

        let ttl_unit = arguments.get("TTL_UNIT");
        let ttl_value = arguments.get("TTL_VALUE");

        if arguments.len() == 2 || ttl_unit.is_none(){
            return Ok(Command::new(
                Set::name(),
                arguments,
            ));
        }

        let ttl_unit: String = ttl_unit.unwrap().clone().try_into().unwrap();

        if ttl_unit.as_str() == "PERSIST" && !ttl_value.is_none() {
            let err = Box::new(CommandParserError::InvalidSyntax);
            return Err(err);
        }

        if ttl_unit.as_str() != "PERSIST" && ttl_value.is_none() {
            let err = Box::new(CommandParserError::InvalidSyntax);
            return Err(err);
        }

        Ok(Command::new(
            Set::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "SET"
    }
}
