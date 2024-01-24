use std::error::Error;

use crate::exe_engine::commands::mset::MSet;
use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct MSetNx;

impl CommandExecutor for MSetNx {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let keys: Vec<String> = cmd.get_required("KEYS")?;

        for key in keys.iter() {
            let entry = storage.read(key)?;
            if !entry.is_nil() {
                let err = Box::new(CommandExecutorError::KeyExisted(key.to_string()));
                return Err(err);
            }
        }

        MSet::execute(storage, cmd)
    }
}
