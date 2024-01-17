use std::collections::VecDeque;

use crate::command_processors::parsers::cores::{CommandTokenizer, CommandTokenizerError};

pub struct StringToCommandTokenizer;

impl CommandTokenizer<String> for StringToCommandTokenizer {
    fn tokenize(&self, value: String) -> Result<VecDeque<String>, Box<CommandTokenizerError>> {
        let mut token = String::new();
        let mut tokens = VecDeque::new();
        let mut stk = Vec::new();

        for chr in value.chars() {
            if stk.is_empty() && chr == ' ' {
                tokens.push_back(token.clone());
                token.clear();
                continue;
            }

            if stk.is_empty() && chr == '"' {
                stk.push(chr);
                continue;
            }

            if chr == '"' && *stk.last().unwrap() == chr {
                tokens.push_back(token.clone());
                token.clear();
                continue;
            }

            token.push(chr);
        }

        if !token.is_empty() {
            tokens.push_back(token);
        }

        Ok(tokens)
    }
}

impl StringToCommandTokenizer {
    pub fn new() -> StringToCommandTokenizer {
        StringToCommandTokenizer {}
    }
}
