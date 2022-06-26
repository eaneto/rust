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
        nodes.push(Node::new(key, value));
    }

    while nodes.len() > 1 {
        nodes.sort_by(|a, b| a.weight.cmp(&b.weight));
        if nodes.len() >= 2 {
            let left = nodes.remove(0);
            let right = nodes.remove(0);
            let weight = left.weight + right.weight;

            let left = Some(Box::new(left));
            let right = Some(Box::new(right));
            let root = Node {
                character: None,
                weight,
                left,
                right,
            };
            nodes.push(root);
        }
    }

    let root = &nodes[0];
    check_tree(root);

    Ok(())
}

fn check_tree(root: &Node) {
    let weight = root.weight;
    let right = &root.right;
    let left = &root.left;
    println!("weight: {}", weight);
    if let Some(n) = left {
        if let Some(c) = n.character {
            println!("left: {}", c);
        }
    }
    if let Some(n) = right {
        if let Some(c) = n.character {
            println!("right: {}", c);
        }
    }
}
