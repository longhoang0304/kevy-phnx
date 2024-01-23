use std::collections::VecDeque;

use crate::command_processors::commands::DecrBy;
use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::Command;

pub struct Decr;

impl CommandParser for Decr {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        if tokens.is_empty() {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(1, 0));
            return Err(err);
        }

        let mut tokens = tokens.clone();
        tokens.push_back(String::from("1"));

        DecrBy::parse(tokens)
    }

    fn name() -> &'static str {
        "DECR"
    }
}
