use std::collections::HashMap;
use std::fmt::Debug;

use super::{CommandArgumentKey, CommandArgumentValue, CommandExecutorError};

pub type CommandArguments = HashMap<CommandArgumentKey, CommandArgumentValue>;

#[derive(Debug)]
pub struct Command {
    pub name: &'static str,
    pub arguments: CommandArguments,
    pub database: u8,
}

impl Command {
    pub fn new(name: &'static str, arguments: CommandArguments) -> Command {
        Command {
            name,
            arguments,
            database: 0,
        }
    }

    pub fn get_argument(
        &self,
        arg_name: &'static str,
        is_required: bool,
    ) -> Result<Option<CommandArgumentValue>, Box<CommandExecutorError>> {
        let args = &self.arguments;
        let arg_value = args.get(arg_name);

        if arg_value.is_none() && is_required {
            let err = CommandExecutorError::MissingRequiredArgument(arg_name.to_string());
            return Err(Box::new(err));
        }

        if arg_value.is_none() {
            return Ok(None);
        }

        let arg_value = arg_value.unwrap().clone();
        Ok(Some(arg_value))
    }

    pub fn get_optional<T>(&self, arg_name: &'static str)
                           -> Result<Option<T>, Box<CommandExecutorError>>
        where CommandArgumentValue: TryInto<T>,
              <CommandArgumentValue as TryInto<T>>::Error: Debug,
    {
        let arg = self.get_argument(arg_name, false)?;
        if arg.is_none() {
            return Ok(None);
        }

        Ok(Some(self.get_required::<T>(arg_name)?))
    }


    pub fn get_required<T>(&self, arg_name: &'static str)
                           -> Result<T, Box<CommandExecutorError>>
        where CommandArgumentValue: TryInto<T>,
              <CommandArgumentValue as TryInto<T>>::Error: Debug,
    {
        let arg = self.get_argument(arg_name, true)?.unwrap().try_into();

        if arg.is_err() {
            println!("{:?}", arg.err());
            let err = CommandExecutorError::InvalidArgumentValue(String::from(arg_name));
            return Err(Box::new(err));
        }

        let value: T = arg.unwrap();

        Ok(value)
    }
}
