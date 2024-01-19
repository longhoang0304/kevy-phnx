use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandParserError {
    WrongNumberOfArguments(u16, u16),
    InvalidArgument(String),
    InvalidArgumentValue(String),
    InvalidSyntax,
}

impl Display for CommandParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandParserError::WrongNumberOfArguments(expected, given) => format!("WrongNumberOfArguments: Wrong number of arguments. Expected: {expected} but {given} was given."),
            CommandParserError::InvalidArgument(arg_name) => format!("InvalidArgument: Invalid argument: {arg_name}."),
            CommandParserError::InvalidArgumentValue(arg_name) => format!("InvalidArgumentValue: Invalid value for argument: {arg_name}."),
            CommandParserError::InvalidSyntax => format!("InvalidSyntax: Invalid Syntax"),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandParserError {}
