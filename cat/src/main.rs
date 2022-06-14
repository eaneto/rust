use std::env;
use std::fs::File;
use std::io::{self, BufRead, ErrorKind};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("{} no such file", filename),
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let lines = io::BufReader::new(file).lines();
    for result in lines {
        match result {
            Ok(line) => println!("{}", line),
            Err(_) => panic!("Error reading file!"),
        };
    }
}
