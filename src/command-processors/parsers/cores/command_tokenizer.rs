use std::collections::VecDeque;

use super::CommandTokenizerError;

pub trait CommandTokenizer<T: Sized> {
    fn tokenize(&self, target: T) -> Result<VecDeque<String>, Box<CommandTokenizerError>>;
}
