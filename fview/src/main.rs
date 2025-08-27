use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("Expected to receive file name from CLI");
    let path = Path::new(&filename);

    let mut file = File::open(path).expect("Unable to open required file");
    let mut buffer = [0; BYTES_PER_LINE];
    let mut pos = 0;

    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        // Writes the current position with a 8 char left pad
        print!("[0x{pos:08x}] ");
        for byte in &buffer {
            match *byte {
                0x00 => print!(". "),
                0xff => print!("## "),
                _ => print!("{byte:02x} "),
            }
        }
        println!();
        pos += BYTES_PER_LINE;
    }
}
