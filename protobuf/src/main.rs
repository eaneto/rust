use protobuf::{create_large_shirt, create_message};

#[tokio::main]
async fn main() {
    let shirt = create_large_shirt("red".to_string());
    dbg!(shirt);
    let message = create_message();
    dbg!(message);
}
