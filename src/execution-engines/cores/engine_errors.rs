use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ExecuteEngineError {}

impl Display for ExecuteEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            _ => "ExecuteEngineErrors",
        };

        write!(f, "{msg}")
    }
}
