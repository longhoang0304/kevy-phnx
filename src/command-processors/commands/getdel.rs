use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct GetDel;

impl CommandParser for GetDel {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len != 1 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(1, tk_len));
            return Err(err);
        }

        let key = CommandArgumentValue::from(tokens[0].clone());
        let arguments = HashMap::from([
            ("KEY", key),
        ]);


        Ok(Command::new(
            GetDel::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "GETDEL"
    }
}
