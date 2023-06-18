use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["proto/items.proto", "proto/message.proto"], &["proto/"])?;
    Ok(())
}
