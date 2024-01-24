use std::collections::{HashMap, VecDeque};

use crate::command_processors::parsers::cores::{CommandParser, CommandParserError};
use crate::exe_engine::cores::{Command, CommandArgumentValue};

pub struct MSet;

enum MSetState {
    KEY,
    VALUE,
}

impl CommandParser for MSet {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>> {
        let tk_len = tokens.len() as u16;
        if tk_len % 2 == 1 {
            let err = Box::new(CommandParserError::WrongNumberOfArguments(tk_len + 1, tk_len));
            return Err(err);
        }

        let mut keys = Vec::new();
        let mut values = Vec::new();
        let mut token_iter = tokens.iter();
        let mut state = MSetState::KEY;

        loop {
            let next = token_iter.next();
            if next.is_none() {
                break;
            }

            let next = next.unwrap().clone();
            match state {
                MSetState::KEY => {
                    keys.push(next);
                    state = MSetState::VALUE;
                }
                MSetState::VALUE => {
                    values.push(next);
                    state = MSetState::KEY;
                }
            }
        }

        let keys = CommandArgumentValue::from(keys);
        let values = CommandArgumentValue::from(values);
        let arguments = HashMap::from([
            ("KEYS", keys),
            ("VALUES", values),
        ]);

        Ok(Command::new(
            Self::name(),
            arguments,
        ))
    }

    fn name() -> &'static str {
        "MSET"
    }
}
