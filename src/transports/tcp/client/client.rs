use std::fmt::{Display, Formatter};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use super::configs::ClientCliConfig;

#[derive(Debug)]
pub enum CliClientError {
    ConnectServerError(String),
    SendCommandError(String),
    ReadResponseError(String),
}

impl Display for CliClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            CliClientError::ConnectServerError(msg) => format!("Connection error: {msg}"),
            CliClientError::SendCommandError(msg) => format!("Send command error: {msg}"),
            CliClientError::ReadResponseError(msg) => format!("Read response error: {msg}"),
        };

        write!(f, "{msg}")
    }
}

// =============================

pub struct CliClient {
    pub configs: ClientCliConfig,
    stream: Option<BufReader<TcpStream>>,
}

impl CliClient {
    pub fn new(configs: ClientCliConfig) -> CliClient {
        CliClient {
            configs,
            stream: None,
        }
    }

    pub fn get_cli_prompt(&self) -> String {
        let configs = &self.configs;
        let mut prompt = format!("{} >> ", configs.host);
        if !configs.user.is_none() {
            prompt = format!("{}@{}", configs.user.as_ref().unwrap(), prompt)
        }

        prompt
    }

    pub async fn connect(&mut self) -> Result<bool, CliClientError> {
        let socket = TcpStream::connect(self.configs.get_addr()).await;

        if let Err(err) = socket {
            return Err(CliClientError::ConnectServerError(err.to_string()));
        }

        let stream = socket.unwrap();

        self.stream = Some(BufReader::new(stream));
        Ok(true)
    }

    pub async fn send_command(&mut self, command: &[u8]) -> Result<bool, CliClientError> {
        let stream = self.stream.as_mut().unwrap();

        let res = stream.write_all(command).await;
        let _ = stream.flush().await;

        if let Err(err) = res {
            return Err(CliClientError::SendCommandError(err.to_string()));
        }

        Ok(true)
    }

    pub async fn read_response(&mut self) -> Result<String, CliClientError> {
        let stream = self.stream.as_mut().unwrap();
        let mut response = String::new();

        let res = stream.read_line(&mut response).await;
        if let Err(err) = res {
            return Err(CliClientError::ReadResponseError(err.to_string()));
        }

        Ok(response)
    }
}
