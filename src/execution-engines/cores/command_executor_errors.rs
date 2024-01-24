use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandExecutorError {
    MissingRequiredArgument(String),
    InvalidArgumentValue(String),
    InvalidCommandSyntax,
    UnknownCommand,
    NotSupportedDataType,
    KeyNotFound(String),
    KeyExisted(String),
}

impl Display for CommandExecutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandExecutorError::MissingRequiredArgument(name) => format!("MissingRequiredArgument: Missing required argument `{name}`."),
            CommandExecutorError::InvalidArgumentValue(name) => format!("InvalidArgumentValue: Value for argument `{name}` is not valid."),
            CommandExecutorError::InvalidCommandSyntax => String::from("InvalidCommandSyntax: Invalid syntax."),
            CommandExecutorError::UnknownCommand => String::from("UnknownCommand: Unknown command."),
            CommandExecutorError::NotSupportedDataType => String::from("NotSupportedData: Value's data type is not supported for this command."),
            CommandExecutorError::KeyNotFound(key) => format!("KeyNotFound: `{key}` is not available."),
            CommandExecutorError::KeyExisted(key) => format!("KeyExisted: `{key}` is already available."),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandExecutorError {}
