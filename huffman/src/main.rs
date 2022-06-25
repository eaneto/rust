use std::{env, process};

fn main() {
    let mut args = env::args();
    args.next();
    let filename = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("File not specified");
            process::exit(1);
        }
    };

    if let Err(e) = huffman::run(filename) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
