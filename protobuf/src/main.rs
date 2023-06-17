use protobuf::create_large_shirt;

#[tokio::main]
async fn main() {
    let shirt = create_large_shirt("red".to_string());
    dbg!(shirt);
}
