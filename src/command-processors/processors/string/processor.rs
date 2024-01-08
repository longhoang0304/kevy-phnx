use std::error::Error;

use crate::command_processors::cores::CommandProcessor;
use crate::command_processors::parsers::cores::{CommandParserFactory};
use crate::exe_engine::cores::{CommandResult, ExecuteEngine};

pub struct StringCommandProcessor {
    pub(self) parser: Box<dyn CommandParserFactory<String>>,
    pub execute_engine: Box<dyn ExecuteEngine>,
}

impl CommandProcessor<String> for StringCommandProcessor {
    fn process(&mut self, command: String) -> Result<CommandResult, Box<dyn Error>> {
        let trimmed = command.trim();
        let command = self.parser.parse(trimmed.to_string())?;

        self.execute_engine.execute(command)
    }
}

impl StringCommandProcessor {
    pub fn new(
        parser: Box<dyn CommandParserFactory<String>>,
        execute_engine: Box<dyn ExecuteEngine>
    ) -> StringCommandProcessor {
        StringCommandProcessor {
            parser,
            execute_engine,
        }
    }
}
