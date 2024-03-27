use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(&["resources/request.proto"], &["resources/"])?;
    Ok(())
}
