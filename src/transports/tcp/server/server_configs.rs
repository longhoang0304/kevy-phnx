use std::{fs, io};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ServerConfigs {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_retry_attempts")]
    pub retry_attempts: i8,
}

fn default_port() -> u16 {
    3458
}

fn default_retry_attempts() -> i8 {
    5
}

impl ServerConfigs {
    pub fn from_file(file_name: &str) -> Result<ServerConfigs, io::Error> {
        let config_data = fs::read_to_string(file_name)?;
        toml::from_str(&config_data).expect("Invalid Kiva Phnx config file. Please double-check your config file.")
    }
}
