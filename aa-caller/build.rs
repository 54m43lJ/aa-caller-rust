use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["src/protos/request.proto",
        "src/protos/profile.proto"],
        &["src/protos/"])?;
    Ok(())
}
