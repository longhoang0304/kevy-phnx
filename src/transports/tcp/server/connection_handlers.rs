use crate::command_processors::cores::CommandProcessor;
use crate::command_processors::parsers::string::{StringToCommandParserFactory, StringToCommandTokenizer};
use crate::command_processors::processors::string::StringCommandProcessor;
use crate::exe_engine::engines::SingleThreadExecuteEngine;
use crate::storage::MemoryStorage;

use super::client::Client;

pub async fn handle_connection(mut client: Client) {
    let client_addr = client.get_addr();
    println!("Client {:?} connected!", client_addr);

    let tokenizer = Box::new(StringToCommandTokenizer::new());
    let parser = Box::new(StringToCommandParserFactory::new(tokenizer));
    let storage = Box::new(MemoryStorage::new(None));
    let execute_engine = Box::new(SingleThreadExecuteEngine::new(storage));
    let mut processor = StringCommandProcessor::new(parser, execute_engine);

    loop {
        let raw_command = client.read_command().await;

        if let Err(err) = raw_command {
            eprintln!("Unable to read request from client: {err}. Existing...");
            break;
        }

        let result = processor.process(raw_command.unwrap());
        let msg = match result {
            Err(err) => err.to_string(),
            Ok(res) => res.to_string(),
        };

        let res = client.send_response(format!("{msg}\r\n").as_bytes()).await;
        if let Err(err) = res {
            eprintln!("Unable to send response to client {err}");
            break;
        }
    }

    println!("Client {:?} disconnected!", client_addr);
}
