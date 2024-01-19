use std::error::Error;
use std::time::SystemTime;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct SetEx;

impl CommandExecutor for SetEx {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let ttl_unit: String = cmd.get_required("TTL_UNIT")?;
        let ttl_value: Option<i128> = cmd.get_optional("TTL_VALUE")?;
        let is_persist_ttl = ttl_unit.as_str() == "PERSIST";

        if is_persist_ttl && ttl_value.is_some() {
            let err = CommandExecutorError::InvalidCommandSyntax;
            return Err(Box::new(err));
        }

        if is_persist_ttl {
            return Ok(CommandResult::from(true));
        }

        if ttl_value.is_none() || ttl_value.unwrap() < 1 {
            let err = CommandExecutorError::InvalidArgumentValue(ttl_unit);
            return Err(Box::new(err));
        }

        let mut entry = storage.read(&key)?;
        let ttl_value = ttl_value.unwrap();

        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as i128;
        let ttl = match ttl_unit.as_str() {
            "EX" => ttl_value * 1000,
            "PX" => ttl_value,
            "EXAT" => ttl_value * 1000 - now,
            "PXAT" => ttl_value - now,
            _ => {
                let err = CommandExecutorError::InvalidCommandSyntax;
                return Err(Box::new(err));
            },
        };

        if ttl < 1 {
            let err = CommandExecutorError::InvalidArgumentValue(ttl_unit);
            return Err(Box::new(err));
        }

        entry.set_ttl(Some(ttl));
        storage.write(entry)?;

        Ok(CommandResult::from(true))
    }
}
