use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandError {
    ValueNotFound(String),
    MissingKey,
    InvalidKey,
    UnknownCommand(String),
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandError::ValueNotFound(key) => format!("Value not found for key: `{key}`"),
            CommandError::InvalidKey => String::from("Key is not valid"),
            CommandError::MissingKey => String::from("Key is required"),
            CommandError::UnknownCommand(cmd) => format!("Unknown command: `{cmd}`"),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandError {}