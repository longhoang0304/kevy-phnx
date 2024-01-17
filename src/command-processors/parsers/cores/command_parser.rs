use std::collections::VecDeque;
use std::error::Error;

use crate::exe_engine::cores::Command;

pub trait CommandParser {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<dyn Error>>;

    fn name() -> &'static str;
}
