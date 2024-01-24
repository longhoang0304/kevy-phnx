use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::{Storage, StorageData, StorageEntry};

pub struct IncrByFloat;

impl CommandExecutor for IncrByFloat {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let amount: f64 = cmd.get_required("AMOUNT")?;

        let entry = storage.read(&key)?;
        if entry.is_nil() {
            let err = Box::new(CommandExecutorError::KeyNotFound(key));
            return Err(err);
        }

        if !entry.is_primitive()  {
            let err = Box::new(CommandExecutorError::NotSupportedDataType);
            return Err(err);
        }

        let data = entry.get_data();
        let new_entry = match data {
            StorageData::String(v) => Some(IncrByFloat::process_string(&entry, v, amount)?),
            StorageData::Number(v) => Some(IncrByFloat::process_number(&entry, v, amount)?),
            StorageData::Decimal(v) => Some(IncrByFloat::process_decimal(&entry, v, amount)?),
            _ => None,
        };

        if new_entry.is_none() {
            return Ok(CommandResult::from(false));
        }

        storage.write(new_entry.unwrap())?;
        Ok(CommandResult::from(true))
    }
}

impl IncrByFloat {
    fn process_string(entry: &StorageEntry, value: &String, amount: f64) -> Result<StorageEntry, Box<CommandExecutorError>> {
        let mut new_entry = entry.clone();
        let raw_value = value;

        let value = raw_value.parse::<f64>();
        if value.is_err() {
            let err = Box::new(CommandExecutorError::NotSupportedDataType);
            return Err(err);
        }

        let v = value.unwrap();
        new_entry.set_data(StorageData::from(v + amount));

        Ok(new_entry)
    }

    fn process_number(entry: &StorageEntry, value: &i128, amount: f64) -> Result<StorageEntry, Box<CommandExecutorError>> {
        let mut new_entry = entry.clone();

        new_entry.set_data(StorageData::from((*value as f64) + amount));

        Ok(new_entry)
    }

    fn process_decimal(entry: &StorageEntry, value: &f64, amount: f64) -> Result<StorageEntry, Box<CommandExecutorError>>{
        let mut new_entry = entry.clone();

        new_entry.set_data(StorageData::from(*value + amount));

        Ok(new_entry)
    }
}
