use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandExecutorError {
    MissingRequiredArgument(String),
    InvalidArgumentValue(String),
    InvalidArgumentSyntax(String),
    UnknownCommand,
    NotSupportedDataType,
}

impl Display for CommandExecutorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CommandExecutorError::MissingRequiredArgument(name) => format!("MissingRequiredArgument: Missing required argument `{name}`."),
            CommandExecutorError::InvalidArgumentValue(name) => format!("InvalidArgumentValue: Value for argument `{name}` is not valid."),
            CommandExecutorError::InvalidArgumentSyntax(name) => format!("InvalidArgumentSyntax: Invalid syntax for argument `{name}`."),
            CommandExecutorError::UnknownCommand => String::from("UnknownCommand: Unknown command."),
            CommandExecutorError::NotSupportedDataType => String::from("NotSupportedData: Value's data type is not supported for this command."),
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandExecutorError {}
