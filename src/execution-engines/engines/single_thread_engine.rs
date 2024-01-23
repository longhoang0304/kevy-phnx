use std::error::Error;

use crate::exe_engine::commands::*;
use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult, ExecuteEngine};
use crate::storage::cores::Storage;

pub struct SingleThreadExecuteEngine {
    storage: Box<dyn Storage>,
}

impl ExecuteEngine for SingleThreadExecuteEngine {
    fn execute(&mut self, command: Command) -> Result<CommandResult, Box<dyn Error>> {
        let execute = match command.name {
            "PING" => Ping::execute,
            "GET" => Get::execute,
            "SET" => Set::execute,
            "APPEND" => Append::execute,
            "GETEX" => GetEx::execute,
            "GETDEL" => GetDel::execute,
            "GETRANGE" => GetRange::execute,
            "SETRANGE" => SetRange::execute,
            "INCRBY" => IncrBy::execute,
            "INCRBYFLOAT" => IncrByFloat::execute,
            "DECRBY" => DecrBy::execute,
            "DECRBYFLOAT" => DecrByFloat::execute,
            "STRLEN" => Strlen::execute,
            "LCS" => Lcs::execute,
            _ => Unknown::execute,
        };

        let res = execute(&mut self.storage, &command)?;
        Ok(res)
    }
}

impl SingleThreadExecuteEngine {
    pub fn new(storage: Box<dyn Storage>) -> SingleThreadExecuteEngine {
        SingleThreadExecuteEngine {
            storage
        }
    }
}

