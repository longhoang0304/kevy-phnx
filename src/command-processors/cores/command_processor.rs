use std::error::Error;

use crate::exe_engine::cores::CommandResult;

pub trait CommandProcessor<T> {
    fn process(&mut self, command: T) -> Result<CommandResult, Box<dyn Error>>;
}
