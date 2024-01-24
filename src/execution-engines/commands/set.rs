use std::error::Error;

use crate::exe_engine::cores::{Command, CommandExecutor, CommandExecutorError, CommandResult};
use crate::storage::cores::{Storage, StorageData};

use super::SetEx;

pub struct Set;

enum SetCondition {
    AlwaysSet,
    SetIfNotExist,
    SetIfAlreadyExist,
}

fn string_to_set_condition(v: &String) -> Result<SetCondition, Box<CommandExecutorError>> {
    match v.as_str() {
        "NX" => Ok(SetCondition::SetIfNotExist),
        "XX" => Ok(SetCondition::SetIfAlreadyExist),
        _ => Err(Box::new(CommandExecutorError::InvalidCommandSyntax)),
    }
}

impl CommandExecutor for Set {
    fn execute(storage: &mut Box<dyn Storage>, cmd: &Command) -> Result<CommandResult, Box<dyn Error>> {
        let key: String = cmd.get_required("KEY")?;
        let value: String = cmd.get_required("VALUE")?;

        let ttl_unit: Option<String> = cmd.get_optional("TTL_UNIT")?;
        let ttl_value: Option<i128> = cmd.get_optional("TTL_VALUE")?;

        let set_condition: Option<String> = cmd.get_optional("SET_CONDITION")?;
        let set_condition = set_condition
            .map(|v| string_to_set_condition(&v))
            .unwrap_or(Ok(SetCondition::AlwaysSet))?;

        let return_prev_data: bool = cmd.get_optional("RETURN_OLD_DATA")?.unwrap_or(false);

        let mut entry = storage.read(&key)?;
        let prev_data = entry.get_data().clone();
        let should_write = match set_condition {
            SetCondition::AlwaysSet => true,
            SetCondition::SetIfAlreadyExist => !entry.is_nil(),
            SetCondition::SetIfNotExist => entry.is_nil(),
        };

        if ttl_unit.is_none() && ttl_value.is_some() {
            let err = CommandExecutorError::InvalidCommandSyntax;
            return Err(Box::new(err));
        }

        if !should_write {
            return Ok(CommandResult::Nil);
        }

        entry.set_data(StorageData::from(value));

        if ttl_unit.is_none() {
            entry.set_ttl(None);
        }

        storage.write(entry)?;

        if !ttl_unit.is_none() {
            SetEx::execute(storage, cmd)?;
        }

        if return_prev_data {
            return Ok(CommandResult::from(prev_data));
        }

        Ok(CommandResult::from(true))
    }
}
