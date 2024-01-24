use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct Strlen;

impl CommandParser for Strlen {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        if tokens.is_empty() {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(1, 0));
            return Err(err);
        }

        let key = CommandArgumentValue::from(tokens[0].clone());
        let arguments = HashMap::from([
            ("KEY", key),
        ]);

        Ok(Command::new(
            Strlen::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "STRLEN"
    }
}
