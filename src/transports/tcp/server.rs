use std::io::{BufReader, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Server {
    listener: TcpListener
}

impl Server {
    pub fn new() -> Server {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", 3458))
            .expect("Unable to open server on port 3458");

        Server{
            listener
        }
    }

    pub fn run(&self) {
        println!("Server started at 127.0.0.1:3458");
        for stream in self.listener.incoming() {
            if let Err(err) = stream {
                eprintln!("Error while connecting to client - Err: {:?}", err);
                continue;
            }

            thread::spawn(|| {
                handle_connection(&mut stream.unwrap());
            });
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
    let rdr = stream.try_clone().unwrap();
    let mut reader = BufReader::new(&rdr);
    let client_addr = stream.peer_addr().unwrap();
    println!("Client {:?} connected!", client_addr);

    loop {
        println!("Waiting for client");
        let mut buffer = String::new();
        let _ = reader.read_line(&mut buffer).unwrap();
        let _ = println!("Client: {:?}", buffer.trim());

        println!("Ponging client");
        let _ = stream.write_all("Pong\n".as_bytes());
        let _ = stream.flush();

        break;
    }

    println!("Client {:?} disconnected!", client_addr);
}

