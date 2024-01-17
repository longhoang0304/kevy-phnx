use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandTokenizerError {}

impl Display for CommandTokenizerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            _ => "CommandTokenizerError",
        };

        write!(f, "{msg}")
    }
}

impl Error for CommandTokenizerError {}
