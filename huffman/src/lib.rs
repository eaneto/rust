use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::Write,
};

struct Node {
    character: Option<char>,
    weight: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(character: char, weight: u32) -> Node {
        Node {
            character: Some(character),
            weight,
            left: None,
            right: None,
        }
    }
}

pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(filename)?;

    let mut map = HashMap::new();
    // TODO: Change to something like unicode-segmentation
    for character in file_content.chars() {
        // TODO Create nodes here
        let count = map.entry(character).or_insert(0);
        *count += 1;
    }

    let mut nodes = Vec::new();
    for (key, value) in map {
        nodes.push(Box::new(Node::new(key, value)));
    }

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| a.weight.cmp(&b.weight));
        if nodes.len() >= 2 {
            let left = nodes.remove(0);
            let right = nodes.remove(0);
            let weight = left.weight + right.weight;

            let left = Some(left);
            let right = Some(right);
            let root = Node {
                character: None,
                weight,
                left,
                right,
            };
            nodes.push(Box::new(root));
        }
    }

    let root = nodes.get(0);
    let mut table: HashMap<char, Vec<u8>> = HashMap::new();
    for character in file_content.chars() {
        if table.contains_key(&character) {
            continue;
        }
        // Traverse tree to find the code for the given character
        let mut code = Vec::new();
        build_character_code(root, &character, &mut code);
        table.insert(character, code);
    }

    // TODO Merge the table building and this loop in one.
    let mut encoded_file = Vec::new();
    for character in file_content.chars() {
        match table.get(&character) {
            Some(code) => encoded_file.push(code),
            None => continue,
        }
    }

    let mut compressed_file = File::create(format!("{}.huff", filename))?;

    for vec in encoded_file {
        if let Err(e) = compressed_file.write(vec) {
            panic!("Error writing to compressed file: {}", e);
        }
    }

    Ok(())
}

// TODO Refactor
fn build_character_code<'a>(
    root: Option<&Box<Node>>,
    character: &'a char,
    code: &mut Vec<u8>,
) -> Option<&'a char> {
    if let Some(node) = root {
        match node.character {
            Some(c) => {
                if c == *character {
                    return Some(character);
                } else {
                    return None;
                }
            }
            None => {
                if build_character_code(node.left.as_ref(), character, code).is_some() {
                    code.push(0);
                    return Some(character);
                }

                if build_character_code(node.right.as_ref(), character, code).is_some() {
                    code.push(1);
                    return Some(character);
                }
            }
        }
    }
    None
}
