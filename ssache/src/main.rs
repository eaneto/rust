use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use log::debug;

enum Command {
    // GET key
    Get { key: String },
    // SET key value
    Set { key: String, value: String },
    Unknown,
}

const CRLF: &str = "\r\n";

fn main() {
    env_logger::init();

    let listener = match TcpListener::bind("127.0.0.1:7777") {
        Ok(listener) => listener,
        Err(e) => panic!("Unable to start ssache on port 7777. Error = {:?}", e),
    };

    let mut database: HashMap<String, String> = HashMap::new();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(_) => continue,
        };

        if let Err(_) = handle_connection(stream, &mut database) {
            continue;
        }
    }
}

#[derive(Debug, Clone)]
struct NotEnoughParametersError;

// TODO Rename this error
#[derive(Debug, Clone)]
struct ConnectionError;

fn handle_connection(
    mut stream: TcpStream,
    database: &mut HashMap<String, String>,
) -> Result<(), ConnectionError> {
    let buf_reader = BufReader::new(&mut stream);
    let command_line = buf_reader.lines().next().unwrap().unwrap();
    let command_line = command_line.split_whitespace();
    let command_line: Vec<&str> = command_line.collect();
    if command_line.get(0).is_none() {
        return Err(ConnectionError);
    }

    let command = command_line.get(0).unwrap();
    let command = parse_command(command, command_line, &mut stream);

    if let Err(_) = command {
        return Err(ConnectionError);
    }

    let command = command.unwrap();

    match command {
        Command::Get { key } => match database.get(&key) {
            Some(value) => {
                debug!("found {:?} for {:?}", value, key);
                let response = format!("OK{CRLF}{value}{CRLF}");
                stream.write_all(response.as_bytes()).unwrap();
                Ok(())
            }
            None => {
                debug!("value not found for {:?}", key);
                let response = format!("OK{CRLF}");
                stream.write_all(response.as_bytes()).unwrap();
                Ok(())
            }
        },
        Command::Set { key, value } => {
            database.insert(key, value);
            debug!("value successfully set");
            let response = format!("OK{CRLF}");
            stream.write_all(response.as_bytes()).unwrap();
            Ok(())
        }
        Command::Unknown => {
            debug!("Unknown command");
            let response = format!("ERROR unknown command{CRLF}");
            stream.write_all(response.as_bytes()).unwrap();
            Ok(())
        }
    }
}

fn parse_command(
    command: &str,
    command_line: Vec<&str>,
    stream: &mut TcpStream,
) -> Result<Command, NotEnoughParametersError> {
    if command.eq(&String::from("GET")) {
        if let Some(key) = command_line.get(1) {
            Ok(Command::Get {
                key: key.to_string(),
            })
        } else {
            debug!("not enough parameters for GET command");
            let response = format!("ERROR not enough parameters for GET{CRLF}");
            stream.write_all(response.as_bytes()).unwrap();
            Err(NotEnoughParametersError)
        }
    } else if command.eq(&String::from("SET")) {
        if let (Some(key), Some(value)) = (command_line.get(1), command_line.get(2)) {
            Ok(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            })
        } else {
            debug!("not enough parameters for SET command");
            let response = format!("ERROR not enough parameters for SET{CRLF}");
            stream.write_all(response.as_bytes()).unwrap();
            Err(NotEnoughParametersError)
        }
    } else {
        Ok(Command::Unknown)
    }
}
