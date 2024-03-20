use std::{error::Error, fs::{self, File}, io::Read, path::Path, u8};

use tokio::net::{UnixListener, UnixStream};
use clap::Parser;

#[derive(Parser)]
#[command(version,about)]
struct Opts {
    #[arg(short, long)]
    daemon :bool,
    #[arg(long="logs")]
    logs :bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let runtime_path = Path::new("/tmp/aa-caller");
    let socket_path = Path::join(runtime_path, "socket");
    
    match fs::create_dir(&runtime_path) {
        Ok(_) => {}
        Err(_e) => {
            // eprintln!("File error: {}", _e);
        }
    }

    match fs::remove_file(&socket_path) {
        Ok(_) => {}
        Err(_e) => {
            // eprintln!("File error: {}", _e);
        }
    }

    let socket = UnixListener::bind(socket_path).unwrap();
    loop {
        match socket.accept().await {
            Ok((stream, _addr)) => {
                // _debug_stream(stream).await?;
                process(stream).await?;
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

async fn process(stream: UnixStream) -> Result<(), Box<dyn Error>> {
    stream.readable().await?;
    let mut buf = vec![0; 2048];
    match stream.try_read(&mut buf) {
        Ok(0) => {}
        Ok(n) => {
            buf.truncate(n);
            let req = String::from_utf8(buf).unwrap();
            println!("{}", req);
            match req.as_str() {
                "logs" => {
                    let logs = ["/var/log/audit/audit.log"];
                    get_logs(stream, &logs).await?;
                }
                _ => {}
            }
        }
        Err(_e) => {}
    }
    Ok(())
}

async fn get_logs(stream :UnixStream, logs :&[&str]) -> Result<(), Box<dyn Error>> {
    for fname in logs {
        let mut log = File::open(fname)?;
        let mut buf :Vec<u8> = Vec::new();
        log.read_to_end(&mut buf)?;
        loop {
            stream.writable().await?;
            match stream.try_write(&buf[..]) {
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
    }
    Ok(())
}

async fn _debug_stream(stream: UnixStream) -> Result<(), Box<dyn Error>> {
    stream.readable().await?;
    let mut buf = vec![0; 2048];
    match stream.try_read(&mut buf) {
        Ok(0) => {}
        Ok(n) => {
            buf.truncate(n);
            let output = String::from_utf8(buf).unwrap();
            println!("{}", output);
        }
        Err(_e) => {}
    }
    Ok(())
}
