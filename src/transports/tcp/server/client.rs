use std::fmt::{Display, Formatter};
use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum ClientError {
    // ConnectionError(String),
    SendResponseError(String),
    ReadCommandError(String),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            // ClientError::ConnectionError(msg) => format!("Connection error: {msg}"),
            ClientError::SendResponseError(msg) => format!("Send response error: {msg}"),
            ClientError::ReadCommandError(msg) => format!("Read command error: {msg}"),
        };

        write!(f, "{msg}")
    }
}

pub struct Client {
    pub stream: BufReader<TcpStream>,
    pub socket_addr: SocketAddr,
}

impl Client {
    pub fn new(tcp_stream: TcpStream, socket_addr: SocketAddr) -> Client {
        Client {
            stream: BufReader::new(tcp_stream),
            socket_addr,
        }
    }

    pub fn get_addr(&self) -> String {
        let addr = &self.socket_addr;
        addr.to_string()
    }

    pub async fn send_response(&mut self, command: &[u8]) -> Result<(), ClientError> {
        let stream = &mut self.stream;
        let res = stream.write_all(command).await;
        let _ = stream.flush().await;

        if let Err(err) = res {
            return Err(ClientError::SendResponseError(err.to_string()));
        }

        Ok(())
    }

    pub async fn read_command(&mut self) -> Result<String, ClientError> {
        let stream = &mut self.stream;
        let mut response = String::new();
        let res = stream.read_line(&mut response).await;

        if let Err(err) = res {
            return Err(ClientError::ReadCommandError(err.to_string()));
        }

        Ok(response)
    }
}
