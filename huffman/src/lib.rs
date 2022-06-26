use std::{collections::HashMap, error::Error, fs};

struct Node<'a> {
    character: Option<char>,
    weight: u32,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(character: char, weight: u32) -> Node<'a> {
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

    // nodes.sort_by(|a, b| a.weight.cmp(&b.weight));
    // nodes.reverse();

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
            nodes.push(root);
        } else {
        }
        //let left = nodes.get(0);
        //let right = nodes.get(1);

        //let left_weight = match left {
        //    Some(node) => node.weight,
        //    None => 0,
        //};
        //let right_weight = match right {
        //    Some(node) => node.weight,
        //    None => 0,
        //};
        //let root = Node {
        //    character: None,
        //    weight: left_weight + right_weight,
        //    left,
        //    right,
        //};
        //nodes.push(root);

        //// There will always be one element to be removed.
        ////nodes.remove(0);
        ////if nodes.len() > 1 {
        ////    nodes.remove(0);
        ////}
        //nodes.push(root);

        // nodes.sort_by(|a, b| a.weight.cmp(&b.weight));
    }

    // for node in nodes.iter() {
    //     match node.character {
    //         Some(character) => println!("{} {}", character, node.weight),
    //         None => println!("None"),
    //     }
    // }

    //while !nodes.is_empty() {}

    Ok(())
}
