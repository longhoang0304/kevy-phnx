use std::collections::VecDeque;

use crate::exe_engine::cores::Command;

use super::CommandParserError;

pub trait CommandParser {
    fn parse(tokens: VecDeque<String>) -> Result<Command, Box<CommandParserError>>;

    fn name() -> &'static str;
}
