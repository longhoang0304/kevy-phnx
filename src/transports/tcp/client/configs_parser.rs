use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    MissingOptions(String),
    UnknownOption(String),
    InvalidValue(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ParseError::MissingOptions(options) => format!("Missing required options: {options}"),
            ParseError::UnknownOption(option) => format!("Unknown option: {option}"),
            ParseError::InvalidValue(msg) => format!("{msg}"),
        };

        write!(f, "{msg}")
    }
}

// =============================

#[derive(Debug)]
pub enum Options {
    Version,
    Help,
    Host(String),
    Port(u16),
    Username(String),
    Password(String),
}

pub fn parse_args(args: &[String]) -> Result<Vec<Options>, ParseError> {
    let mut itr = args.iter();

    // -v, -h
    if args.len() == 1 {
        let opt = itr.next().unwrap().clone();
        let opt = opt.to_lowercase();

        return match opt.trim() {
            "-v" | "--version" => Ok(vec![Options::Version]),
            "-h" | "--help" => Ok(vec![Options::Help]),
            _ => Err(ParseError::UnknownOption(opt.clone())),
        };
    }

    let mut ops = Vec::new();

    for (idx, opt) in itr.clone().step_by(2).enumerate() {
        let value = itr.nth(idx + 1).unwrap().clone();
        match opt.trim() {
            "-u" | "--username" => ops.push(Options::Username(value)),
            "-p" | "--password" => ops.push(Options::Password(value)),
            "-H" | "--host" => ops.push(Options::Host(value)),
            "-P" | "--port" => {
                let v = value.parse::<u16>();
                if v.is_err() {
                    return Err(ParseError::InvalidValue(format!("Invalid value for port. Expected number (0-65k) but {value} was found")));
                }
                ops.push(Options::Port(v.unwrap()));
            }
            _ => return Err(ParseError::UnknownOption(opt.clone())),
        }
    }

    Ok(ops)
}
