use std::{collections::HashMap, error::Error, fs};

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

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
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
    let mut table: HashMap<char, String> = HashMap::new();
    let mut encoded_file = String::new();
    for character in file_content.chars() {
        if let Some(c) = table.get(&character) {
            encoded_file.push_str(c);
            continue;
        }
        // Traverse tree to find the code for the given character
        let mut code = String::new();
        build_character_code(root, &character, &mut code);
        println!("{}: {}", character, code);
        encoded_file.push_str(&code);
        table.insert(character, code);
    }

    println!("{}", encoded_file);

    Ok(())
}

// TODO Refactor
fn build_character_code<'a>(
    root: Option<&Box<Node>>,
    character: &'a char,
    code: &mut String,
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
                if let Some(_) = build_character_code(node.left.as_ref(), character, code) {
                    code.push('0');
                    return Some(character);
                }

                if let Some(_) = build_character_code(node.right.as_ref(), character, code) {
                    code.push('1');
                    return Some(character);
                }
            }
        }
    }
    None
}
