use std::net::TcpListener;
use std::thread::sleep;
use std::time::Duration;

use crate::server::server_configs::ServerConfigs;

pub struct Server;

impl Server {
    pub fn new_server(server_config: &ServerConfigs) -> TcpListener {
        let port = server_config.port;
        let max_attempts = server_config.retry_attempts;
        let err_msg = format!("Unable to open server on port {port}. - Error {err}");
        let mut current_attempt = 0;

        while current_attempt <= max_attempts {
            let tcp_result = TcpListener::bind(format!("0.0.0.0:{}", port));
            current_attempt += 1;

            if let Err(err) = tcp_result {
                let dur = 1 * current_attempt as u64;

                eprintln!("{}", err_msg);
                eprintln!("Retrying after {dur}s...");
                sleep(Duration::from_secs(dur));
                continue;
            }

            tcp_result.unwrap()
        }

        panic!(err_msg);
    }

    pub fn listen(&self) {

    }
}
