use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

use log::debug;

enum Command {
    // GET key
    Get { key: String },
    // SET key value
    Set { key: String, value: String },
    Unknown,
}
fn main() {
    env_logger::init();

    let listener = match TcpListener::bind("127.0.0.1:7777") {
        Ok(listener) => listener,
        Err(e) => panic!("Unable to start ssache on port 7777. Error = {:?}", e),
    };

    let mut database = HashMap::new();

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
        };

        let buf_reader = BufReader::new(&mut stream);
        let command_line = buf_reader.lines().next().unwrap().unwrap();
        let command_line = command_line.split_whitespace();
        let command_line: Vec<&str> = command_line.collect();
        let command = match command_line.get(0) {
            Some(command) => command,
            None => continue,
        };

        // TODO Respond to client error for not enough parameters
        let command = if command.eq(&String::from("GET")) {
            let key = match command_line.get(1) {
                Some(key) => key,
                None => {
                    debug!("not enough parameters for GET command");
                    continue;
                }
            };
            Command::Get {
                key: key.to_string(),
            }
        } else if command.eq(&String::from("SET")) {
            let key = match command_line.get(1) {
                Some(key) => key,
                None => {
                    debug!("not enough parameters for SET command");
                    continue;
                }
            };
            let value = match command_line.get(2) {
                Some(value) => value,
                None => {
                    debug!("not enough parameters for SET command");
                    continue;
                }
            };
            Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }
        } else {
            Command::Unknown
        };

        match command {
            Command::Get { key } => match database.get(&key) {
                Some(value) => {
                    debug!("found {:?} for {:?}", value, key);
                    let response = format!("OK\r\n{value}\r\n");
                    stream.write_all(response.as_bytes()).unwrap();
                }
                None => {
                    debug!("value not found for {:?}", key);
                    let response = format!("OK\r\n");
                    stream.write_all(response.as_bytes()).unwrap();
                }
            },
            Command::Set { key, value } => {
                database.insert(key, value);
                debug!("value successfully set");
                let response = format!("OK\r\n");
                stream.write_all(response.as_bytes()).unwrap();
            }
            Command::Unknown => {
                debug!("Unknown command");
                let response = format!("ERROR unknown command\r\n");
                stream.write_all(response.as_bytes()).unwrap();
            }
        }
    }
}
