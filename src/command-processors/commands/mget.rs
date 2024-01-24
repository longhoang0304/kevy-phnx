use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct MGet;

impl CommandParser for MGet {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        if tokens.is_empty() {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(1, 0));
            return Err(err);
        }

        let mut keys = Vec::new();
        let mut token_iter = tokens.iter();

        loop {
            let next = token_iter.next();
            if next.is_none() {
                break;
            }

            let next = next.unwrap().clone();
            keys.push(next);
        }

        let keys = CommandArgumentValue::from(keys);
        let arguments = HashMap::from([
            ("KEYS", keys),
        ]);

        Ok(Command::new(
            MGet::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "MGET"
    }
}
