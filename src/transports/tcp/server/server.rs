use std::fmt::{Display, Formatter};

use tokio;
use tokio::net::TcpListener;

use super::client::Client;

#[derive(Debug)]
pub enum ServerError {
    OpenServerError(String),
    AcceptClientError(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let err_msg = match self {
            ServerError::OpenServerError(msg) => format!("Cannot open server due to error: {msg}"),
            ServerError::AcceptClientError(msg) => format!("Cannot connect to client due to error: {msg}"),
        };

        write!(f, "{}", err_msg)
    }
}


#[derive(Debug)]
pub struct Server {
    listener: Option<TcpListener>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            listener: None
        }
    }

    pub async fn open(&mut self) -> Result<(), ServerError> {
        let listener = TcpListener::bind("127.0.0.1:3458").await;

        if let Err(err) = listener {
            return Err(ServerError::OpenServerError(err.to_string()));
        }

        self.listener = Some(listener.unwrap());
        Ok(())
    }

    pub async fn accept(&self) -> Result<Client, ServerError> {
        let res = self.listener.as_ref().unwrap().accept().await;
        if let Err(err) = res {
            return Err(ServerError::AcceptClientError(err.to_string()));
        }

        let (stream, sock) = res.unwrap();
        Ok(Client::new(stream, sock))
    }
}
