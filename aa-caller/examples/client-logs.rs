
use std::{error::Error, process::Command};

use tokio::{io::AsyncWriteExt, net::UnixStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Command::new("echo")
        .args(["\"This is a test\"", ">", "/tmp/test.log"])
        .output()
        .expect("command failed!");
    let path = "/tmp/aa-caller/socket";
    let mut conn = UnixStream::connect(path).await?;
    loop {
        conn.writable().await?;
        match conn.try_write(b"logs") {
            Ok(0) => {
                continue;
            }
            Ok(_n) => {
                break;
            }
            Err(_e) => {
                continue;
            }
        }
    }
    loop {
        conn.readable().await?;
        let mut buf = vec![0; 20000];
        match conn.try_read(&mut buf) {
            Ok(0) => {
                break;
            }
            Ok(n) => {
                println!("{} bytes of logs received", n);
                let out = String::from_utf8(buf[..n].to_vec()).unwrap();
                println!("{}", out);
                break;
            }
            Err(_e) => {
                continue;
            }
        }
    }
    conn.shutdown().await?;
    Ok(())
}
