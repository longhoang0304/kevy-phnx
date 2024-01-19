use std::error::Error;
use std::time::SystemTime;

use crate::exe_engine::commands::funcs::{get_optional, get_required};
use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct GetEx;

impl CommandExecutor for GetEx {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = get_required("KEY", cmd)?;
        let ttl_unit: String = get_required("TTL_UNIT", cmd)?;
        let ttl_value: Option<i128> = get_optional("TTL_VALUE", cmd)?;
        let is_persist_ttl = ttl_unit.as_str() == "PERSIST";

        if is_persist_ttl && ttl_value.is_some() {
            let err = CommandExecutorError::InvalidArgumentSyntax(ttl_unit);
            return Err(Box::new(err));
        }

        let mut entry = storage.read(&key)?;
        let data = entry.get_data().clone();

        if is_persist_ttl {
            return Ok(CommandResult::from(data));
        }

        let ttl_value = ttl_value.unwrap();
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as i128;
        let ttl = match ttl_unit.as_str() {
            "EX" => Some(ttl_value * 1000),
            "PX" => Some(ttl_value),
            "EXAT" => Some(ttl_value * 1000 - now),
            "PXAT" => Some(ttl_value - now),
            _ => None,
        };

        entry.set_ttl(ttl);
        storage.write(entry)?;

        Ok(CommandResult::from(data))
    }
}
