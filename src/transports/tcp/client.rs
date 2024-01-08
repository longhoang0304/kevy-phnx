use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream};

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new() -> Client {
        let stream = TcpStream::connect(format!("127.0.0.1:{}", 3458))
            .expect("Unable to open client on port 3458");

        Client{
            stream,
        }
    }

    pub fn run(&mut self) {
        println!("Connected to the server at 127.0.0.1:3458");
        let mut reader = BufReader::new(&self.stream);
        let mut writer = self.stream.try_clone().unwrap();

        loop {
            println!("Pinging server");
            let _ = writer.write_all("Ping\n".as_bytes());
            let _ = writer.flush();

            println!("Waiting for response");
            let mut buffer = String::new();
            let _ = reader.read_line(&mut buffer).unwrap();
            println!("Server: {:?}", buffer.trim());

            break;
        }

        println!("Disconnected from server");
    }
}
