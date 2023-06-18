// Include the `items` module, which is generated from items.proto.
// It is important to maintain the same structure as in the proto.
pub mod example {
    pub mod proto {
        include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));
        include!(concat!(env!("OUT_DIR"), "/example.message.rs"));
    }
}

use example::proto;
use prost_types::Timestamp;

pub fn create_large_shirt(color: String) -> proto::Shirt {
    let mut shirt = example::proto::Shirt {
        color,
        ..Default::default()
    };
    shirt.set_size(proto::shirt::Size::Large);
    shirt
}

pub fn create_message() -> proto::Message {
    let timestamp = Timestamp::date(2023, 6, 18).unwrap();
    proto::Message {
        id: 0,
        log_name: "name".to_string(),
        partition: 10,
        data: Vec::new(),
        timestamp: Some(timestamp),
    }
}
