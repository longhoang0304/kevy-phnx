use std::fmt::Debug;
use crate::exe_engine::cores::{Command, CommandArgumentValue, CommandExecutorError};

use super::get_argument;

pub fn get_required<T>(arg_name: &'static str, cmd: &Command)
    -> Result<T, Box<CommandExecutorError>>
    where CommandArgumentValue: TryInto<T>,
          <CommandArgumentValue as TryInto<T>>::Error: Debug,
{
    let arg = get_argument(arg_name, true, cmd)?.unwrap().try_into();

    if arg.is_err() {
        let err = CommandExecutorError::InvalidArgumentValue(String::from(arg_name));
        return Err(Box::new(err));
    }

    let value: T = arg.unwrap();

    Ok(value)
}
