use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandExecutorError {
    MissingKey,
    InvalidKey,
    InvalidArgument,
    WrongCommandType,
    UnknownCommand(String),
}

impl Display for CommandExecutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandExecutorError::InvalidKey => String::from("InvalidKey"),
            CommandExecutorError::MissingKey => String::from("MissingKey"),
            CommandExecutorError::InvalidArgument => String::from("InvalidArgument"),
            CommandExecutorError::WrongCommandType => String::from("WrongCommandType: The given key doesn't support this command."),
            CommandExecutorError::UnknownCommand(cmd) => format!("UnknownCommand: `{cmd}`"),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandExecutorError {}
