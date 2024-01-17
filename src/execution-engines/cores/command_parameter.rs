use crate::exe_engine::cores::CommandExecutorError;

#[derive(Debug, Clone)]
pub struct CommandParameterPair(pub String, pub Box<CommandParameter>);

#[derive(Debug, Clone)]
pub enum CommandParameter {
    Decimal(f64),
    Number(i128),
    String(String),
    Pair(CommandParameterPair),
}

impl TryInto<String> for CommandParameter {
    type Error = Box<CommandExecutorError>;

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            CommandParameter::String(v) => Ok(v),
            _ => Err(Box::new(CommandExecutorError::InvalidArgument)),
        }
    }
}

impl TryInto<i128> for CommandParameter {
    type Error = Box<CommandExecutorError>;

    fn try_into(self) -> Result<i128, Self::Error> {
        match self {
            CommandParameter::Number(v) => Ok(v),
            _ => Err(Box::new(CommandExecutorError::InvalidArgument)),
        }
    }
}

impl TryInto<f64> for CommandParameter {
    type Error = Box<CommandExecutorError>;

    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            CommandParameter::Decimal(v) => Ok(v),
            _ => Err(Box::new(CommandExecutorError::InvalidArgument)),
        }
    }
}

impl TryInto<CommandParameterPair> for CommandParameter {
    type Error = Box<CommandExecutorError>;

    fn try_into(self) -> Result<CommandParameterPair, Self::Error> {
        match self {
            CommandParameter::Pair(v) => Ok(v),
            _ => Err(Box::new(CommandExecutorError::InvalidArgument)),
        }
    }
}

// =========
impl From<String> for CommandParameter {
    fn from(value: String) -> Self {
        CommandParameter::String(value)
    }
}

impl From<&String> for CommandParameter {
    fn from(value: &String) -> Self {
        CommandParameter::String(value.clone())
    }
}

impl From<i128> for CommandParameter {
    fn from(value: i128) -> Self {
        CommandParameter::Number(value)
    }
}

impl From<f64> for CommandParameter {
    fn from(value: f64) -> Self {
        CommandParameter::Decimal(value)
    }
}

impl From<CommandParameterPair> for CommandParameter {
    fn from(value: CommandParameterPair) -> Self {
        CommandParameter::Pair(value)
    }
}

impl From<Box<CommandParameter>> for CommandParameter {
    fn from(value: Box<CommandParameter>) -> Self {
        match *value {
            CommandParameter::Number(v) => CommandParameter::Number(v),
            CommandParameter::String(v) => CommandParameter::String(v),
            CommandParameter::Decimal(v) => CommandParameter::Decimal(v),
            CommandParameter::Pair(v) => CommandParameter::Pair(v),
        }
    }
}
