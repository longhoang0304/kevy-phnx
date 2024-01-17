use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandParserError {
    WrongNumberOfArguments(u16, u16),
    InvalidArgument(String),
    InvalidArgumentValue(String),
}

impl Display for CommandParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandParserError::WrongNumberOfArguments(expected, given) => format!("Wrong number of arguments. Expected: {expected} but {given} was given."),
            CommandParserError::InvalidArgument(arg_name) => format!("Invalid argument: {arg_name}."),
            CommandParserError::InvalidArgumentValue(arg_name) => format!("Invalid value for argument: {arg_name}."),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandParserError {}
