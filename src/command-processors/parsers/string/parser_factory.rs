use std::error::Error;

use crate::command_processors::parsers::commands::{Ping, Set, Unknown};
use crate::command_processors::parsers::commands::Get;
use crate::command_processors::parsers::cores::{CommandParser, CommandParserFactory, CommandTokenizer};
use crate::exe_engine::cores::Command;

pub struct StringToCommandParserFactory {
    tokenizer: Box<dyn CommandTokenizer<String>>,
}

impl CommandParserFactory<String> for StringToCommandParserFactory {
    fn parse(&self, target: String) -> Result<Command, Box<dyn Error>> {
        let mut tokens = self.tokenizer.as_ref().tokenize(target)?;
        let name = tokens.pop_front().unwrap().to_uppercase();

        let parse = match name.as_str() {
            "GET" => Get::parse,
            "SET" => Set::parse,
            "PING" => Ping::parse,
            _ => Unknown::parse,
        };

        parse(tokens)
    }
}

impl StringToCommandParserFactory {
    pub fn new(tokenizer: Box<dyn CommandTokenizer<String>>) -> StringToCommandParserFactory {
        StringToCommandParserFactory {
            tokenizer
        }
    }
}