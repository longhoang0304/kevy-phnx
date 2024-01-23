use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct Lcs;

impl CommandParser for Lcs {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len < 2 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(2, tk_len));
            return Err(err);
        }

        let mut token_iter = tokens.iter();
        let key1 = CommandArgumentValue::from(token_iter.next().unwrap().clone());
        let key2 = CommandArgumentValue::from(token_iter.next().unwrap().clone());
        let mut arguments = HashMap::from([
            ("KEY1", key1),
            ("KEY2", key2),
        ]);

        loop {
            let next = token_iter.next();
            if next.is_none() {
                break;
            }

            let next = next.map(|e| e.to_uppercase()).unwrap();

            match next.as_str() {
                "LEN" if !arguments.contains_key("LENGTH_ONLY") =>
                    arguments.insert("LENGTH_ONLY", CommandArgumentValue::from(true)),

                _ => {
                    let err = Box::new(CommandParserError::InvalidSyntax);
                    return Err(err);
                }
            };
        }

        Ok(Command::new(
            Lcs::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "LCS"
    }
}

