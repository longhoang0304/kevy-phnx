use super::configs_parser::{Options, ParseError};

const DEFAULT_PORT: u16 = 3458;

pub struct ClientCliConfig {
    pub host: String,
    pub port: u16,
    pub user: Option<String>,
    pub pass: Option<String>,
}

impl ClientCliConfig {
    pub fn new(host: String, port: Option<u16>, user: Option<String>, pass: Option<String>) -> ClientCliConfig {
        ClientCliConfig {
            host,
            user,
            pass,
            port: port.unwrap_or(DEFAULT_PORT),
        }
    }

    pub fn from_options(options: Vec<Options>) -> Result<ClientCliConfig, ParseError> {
        let mut host: Option<String> = None;
        let mut user: Option<String> = None;
        let mut pass: Option<String> = None;
        let mut port: Option<u16> = None;

        for opt in options {
            match opt {
                Options::Host(h) => host = Some(h),
                Options::Port(p) => port = Some(p),
                Options::Username(u) => user = Some(u),
                Options::Password(pw) => pass = Some(pw),
                _ => ()
            };
        }

        if host.is_none() {
            return Err(ParseError::MissingOptions(String::from("head")));
        }

        Ok(ClientCliConfig::new(host.unwrap(), port, user, pass))
    }

    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
