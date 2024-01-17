use std::error::Error;
use std::time::SystemTime;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandResult};
use crate::storage::cores::Storage;

pub struct GetEx;

impl CommandExecutor for GetEx {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let parameters = cmd.parameters.as_ref().unwrap();
        let mut params_iter = parameters.iter();
        let key: String = params_iter.next().unwrap().clone().try_into()?;
        let time_unit: String = params_iter.next().unwrap().clone().try_into()?;
        let time_value: i128 = params_iter.next().unwrap().clone().try_into()?;
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_millis() as i128;

        let ttl = match time_unit.as_str() {
            "EX" => Some(time_value * 1000),
            "PX" => Some(time_value),
            "EXAT" => Some(time_value * 1000 - now),
            "PXAT" => Some(time_value - now),
            _ => None,
        };

        let mut entry = storage.read(&key)?;
        let data = entry.value.data.clone();

        entry.set_ttl(ttl);
        storage.write(entry)?;

        Ok(CommandResult::from(data))
    }
}
