use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandParseError {
    EmptyQuery,
    MissingParametersForCommand(&'static str),
    InvalidParametersForCommand(&'static str),
}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandParseError::EmptyQuery => String::from("Empty query"),
            CommandParseError::MissingParametersForCommand(cmd) => format!("Missing parameters for command: {cmd}"),
            CommandParseError::InvalidParametersForCommand(cmd) => format!("Invalid parameter for command: {cmd}"),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandParseError {}
