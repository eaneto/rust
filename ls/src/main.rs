use std::env;
use std::fs;
use std::fs::ReadDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 { &args[1] } else { "." };

    match fs::metadata(directory) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                panic!("Path should be a directory.");
            }
        }
        Err(_) => panic!("Unable to read file metadata"),
    }

    match fs::read_dir(directory) {
        Ok(directory) => display_directory_contents(directory),
        Err(_) => panic!("Unable to read directory"),
    }
}

fn display_directory_contents(directory: ReadDir) {
    for entry in directory {
        match entry {
            Ok(file) => println!("{:?}", file.path()),
            Err(_) => panic!("Unable to read file"),
        }
    }
}
