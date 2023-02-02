use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpListener,
};

enum Command {
    // GET key
    Get { key: String },
    // SET key value
    Set { key: String, value: String },
    Unknown,
}
fn main() {
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
        let command = if command.eq(&String::from("GET")) {
            let key = match command_line.get(1) {
                Some(key) => key,
                None => continue,
            };
            Command::Get {
                key: key.to_string(),
            }
        } else if command.eq(&String::from("SET")) {
            let key = match command_line.get(1) {
                Some(key) => key,
                None => continue,
            };
            let value = match command_line.get(2) {
                Some(value) => value,
                None => continue,
            };
            Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }
        } else {
            Command::Unknown
        };

        // TODO Add logging
        // TODO Return data to the client
        match command {
            Command::Get { key } => match database.get(&key) {
                Some(value) => println!("found {}", value),
                None => println!("not found"),
            },
            Command::Set { key, value } => {
                database.insert(key, value);
            }
            Command::Unknown => {
                println!("Unknown command");
            }
        }
    }
}
