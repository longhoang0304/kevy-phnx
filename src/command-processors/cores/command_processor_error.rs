use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CommandProcessorError {

}

impl Display for CommandProcessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            _ => "CommandProcessorErrors"
        };

        write!(f, "{msg}")
    }
}
