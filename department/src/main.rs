use std::{collections::HashMap, io};

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Input command: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let input = input.trim().split_whitespace().collect::<Vec<&str>>();

        let person = input[1];
        let department = input[3];

        departments
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(person.to_string());

        for (name, people) in &departments {
            println!("{}", name);
            for person in people {
                println!("{}", person);
            }
        }
    }
}
