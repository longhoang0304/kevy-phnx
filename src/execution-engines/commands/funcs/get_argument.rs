use crate::exe_engine::cores::{Command, CommandArgumentValue, CommandExecutorError};

pub fn get_argument<'a>(
    arg_name: &'static str,
    is_required: bool,
    command: &Command
) -> Result<Option<CommandArgumentValue>, Box<CommandExecutorError>> {
    let args = &command.arguments;
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
