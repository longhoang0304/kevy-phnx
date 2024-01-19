use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::Storage;

pub struct GetRange;

impl CommandExecutor for GetRange {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let mut start: i128 = cmd.get_required("START")?;
        let mut end: i128 = cmd.get_required("END")?;

        let entry = storage.read(&key)?;

        if !entry.is_primitive() {
            let err = Box::new(CommandExecutorError::NotSupportedDataType);
            return Err(err);
        }

        let data = entry.get_data();
        let data = data.to_string();
        let data_len = data.len() as i128;

        if start >= data_len {
            return Ok(CommandResult::from(""));
        }

        if start < 0 {
            start = data_len + start;
        }

        if end < 0 {
            end = data_len + end;
        }

        if end < start || end < 0 {
            return Ok(CommandResult::from(""));
        }

        if start < 0 {
            start = 0;
        }

        if end > data_len {
            end = data_len;
        }

        let start: usize = start as usize;
        let end: usize = end as usize;
        let result = &data[start..=end];

        Ok(CommandResult::from(result))
    }
}
