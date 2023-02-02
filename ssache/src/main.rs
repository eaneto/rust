use std::collections::HashMap;

enum Command {
    Get { key: String },
    Set { key: String, value: String },
}
fn main() {
    // TODO Open TCP Connection
    // TODO Read TCP packets
    // TODO Process Get and Set commands
    let command = Command::Get {
        key: "example".to_string(),
    };

    let mut database = HashMap::new();

    match command {
        Command::Get { key } => match database.get(&key) {
            Some(value) => println!("found {}", value),
            None => println!("not found"),
        },
        Command::Set { key, value } => {
            database.insert(key, value);
        }
    }
}
