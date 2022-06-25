use std::{collections::HashMap, error::Error, fs};

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(filename)?;

    let mut map = HashMap::new();
    for character in file_content.chars() {
        let count = map.entry(character).or_insert(0);
        *count += 1;
    }

    for (key, value) in &map {
        println!("{} {}", key, value);
    }

    Ok(())
}
