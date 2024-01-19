use super::CommandArgumentValueError;

pub type CommandArgumentKey = &'static str;

#[derive(Debug, Clone)]
pub enum CommandArgumentValue {
    Decimal(f64),
    Number(i128),
    String(String),
    Bool(bool),
}

impl TryInto<String> for CommandArgumentValue {
    type Error = CommandArgumentValueError;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            CommandArgumentValue::String(v) => Ok(v),
            _ => Err(CommandArgumentValueError::TryIntoError),
        }
    }
}

impl TryInto<i128> for CommandArgumentValue {
    type Error = CommandArgumentValueError;

    fn try_into(self) -> Result<i128, Self::Error> {
        match self {
            CommandArgumentValue::Number(v) => Ok(v),
            _ => Err(CommandArgumentValueError::TryIntoError),
        }
    }
}

impl TryInto<f64> for CommandArgumentValue {
    type Error = CommandArgumentValueError;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            CommandArgumentValue::Decimal(v) => Ok(v),
            _ => Err(CommandArgumentValueError::TryIntoError),
        }
    }
}

impl TryInto<bool> for CommandArgumentValue {
    type Error = CommandArgumentValueError;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            CommandArgumentValue::Bool(v) => Ok(v),
            _ => Err(CommandArgumentValueError::TryIntoError),
        }
    }
}

// =========

impl From<String> for CommandArgumentValue {
    fn from(value: String) -> Self {
        CommandArgumentValue::String(value)
    }
}

impl From<&String> for CommandArgumentValue {
    fn from(value: &String) -> Self {
        CommandArgumentValue::String(value.clone())
    }
}

impl From<i128> for CommandArgumentValue {
    fn from(value: i128) -> Self {
        CommandArgumentValue::Number(value)
    }
}

impl From<f64> for CommandArgumentValue {
    fn from(value: f64) -> Self {
        CommandArgumentValue::Decimal(value)
    }
}

impl From<bool> for CommandArgumentValue {
    fn from(value: bool) -> Self {
        CommandArgumentValue::Bool(value)
    }
}

impl From<Box<CommandArgumentValue>> for CommandArgumentValue {
    fn from(value: Box<CommandArgumentValue>) -> Self {
        *value
    }
}

// =========
