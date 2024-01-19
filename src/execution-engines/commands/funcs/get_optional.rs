use std::fmt::Debug;
use crate::exe_engine::cores::{Command, CommandArgumentValue, CommandExecutorError};

use super::{get_argument, get_required};

pub fn get_optional<T>(arg_name: &'static str, cmd: &Command)
    -> Result<Option<T>, Box<CommandExecutorError>>
    where CommandArgumentValue: TryInto<T>,
        <CommandArgumentValue as TryInto<T>>::Error: Debug,
{
    let arg = get_argument(arg_name, true, cmd)?;
    if arg.is_none() {
        return Ok(None);
    }

    Ok(Some(get_required(arg_name, cmd)?))
}
