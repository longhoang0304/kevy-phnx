use std::error::Error;

use crate::exe_engine::cores::Command;

pub trait CommandParserFactory<T: Sized> {
    fn parse(&self, target: T) -> Result<Command, Box<dyn Error>>;
}