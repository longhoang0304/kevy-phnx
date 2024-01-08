use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            _ => "StorageError",
        };

        write!(f, "{msg}")
    }
}

impl Error for StorageError {}
