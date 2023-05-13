use std::{fs, thread, time::Duration};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878").await {
        Ok(listener) => listener,
        Err(_) => panic!("Unable to bind port 7878"),
    };

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(_) => continue,
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    let result = buf_reader.read_line(&mut request_line).await;
    if result.is_err() {
        return;
    }

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1\r\n" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1\r\n" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    let _ = stream.write_all(response.as_bytes()).await;
}
