use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct MGet;

impl CommandExecutor for MGet {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let keys: Vec<String> = cmd.get_required("KEYS")?;
        let mut data = Vec::new();

        for key in keys.iter() {
            let entry = storage.read(key)?;
            let d = entry.get_data();
            data.push(d.to_string());
        }

        Ok(CommandResult::from(data))
    }
}
