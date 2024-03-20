use std::error::Error;

use tokio::net::UnixStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = "/tmp/aa-caller/socket";
    let conn = UnixStream::connect(path).await?;
    loop {
        conn.writable().await?;
        match conn.try_write(b"I did it!") {
            Ok(_n) => {
                break;
            }
            Err(_e) => {
                continue;
            }
        }
    }
    Ok(())
}
