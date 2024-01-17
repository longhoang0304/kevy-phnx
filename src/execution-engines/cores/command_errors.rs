use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
    MissingKey,
    InvalidKey,
    InvalidArgument,
    WrongCommandType,
    UnknownCommand(String),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandError::InvalidKey => String::from("InvalidKey"),
            CommandError::MissingKey => String::from("MissingKey"),
            CommandError::InvalidArgument => String::from("InvalidArgument"),
            CommandError::WrongCommandType => String::from("WrongCommandType: The given key doesn't support this command."),
            CommandError::UnknownCommand(cmd) => format!("UnknownCommand: `{cmd}`"),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandError {}