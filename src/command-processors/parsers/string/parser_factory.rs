use std::error::Error;

use crate::command_processors::commands::*;
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
            "APPEND" => Append::parse,
            "GETEX" => GetEx::parse,
            "GETDEL" => GetDel::parse,
            "GETRANGE" => GetRange::parse,
            "SET" => Set::parse,
            "SETRANGE" => SetRange::parse,
            "PING" => Ping::parse,
            "INCR" => Incr::parse,
            "INCRBY" => IncrBy::parse,
            "INCRBYFLOAT" => IncrByFloat::parse,
            "DECR" => Decr::parse,
            "DECRBY" => DecrBy::parse,
            "DECRBYFLOAT" => DecrByFloat::parse,
            _ => Unknown::parse,
        };

        Ok(parse(tokens)?)
    }
}

impl StringToCommandParserFactory {
    pub fn new(tokenizer: Box<dyn CommandTokenizer<String>>) -> StringToCommandParserFactory {
        StringToCommandParserFactory {
            tokenizer
        }
    }
}
