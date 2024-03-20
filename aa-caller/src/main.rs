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

static LOG_FILES:[&str; 1] = ["/var/log/audit/audit.log"];
// static LOG_FILES:[&str; 1] = ["/tmp/test.log"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    if opts.logs {
        for l in LOG_FILES {
            println!("{}", String::from_utf8(get_logs(l).unwrap()).unwrap());
        }
    } else if opts.daemon {
    } else {
        listen().await?;
    }
    Ok(())
}

async fn listen() -> Result<(), Box<dyn Error>> {
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
            Ok((stream,_addr)) => {
                // _debug_stream(stream).await?;
                process(&stream).await?;
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}

async fn process(stream: &UnixStream) -> Result<(), Box<dyn Error>> {
    stream.readable().await?;
    let mut buf = vec![0; 2048];
    match stream.try_read(&mut buf) {
        Ok(0) => {
        }
        Ok(n) => {
            buf.truncate(n);
            let req = String::from_utf8(buf).unwrap();
            match req.as_str() {
                "logs" => {
                    println!("logs requested");
                    for l in LOG_FILES {
                        let log = get_logs(&l).unwrap();
                         println!("Log:\n {}", String::from_utf8(log[..].to_vec()).unwrap());
                        stream_write(stream, &log[..]).await?;
                    }
                }
                _ => {}
            }
        }
        Err(_e) => {}
    }
    Ok(())
}

/// fetch logs from a kernel log file
fn get_logs(fname :&str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut log = File::open(fname)?;
    let mut buf :Vec<u8> = Vec::new();
    let n = log.read_to_end(&mut buf)?;
    buf.truncate(n);
    Ok(buf)
}

/// write any slice of byte string to the stream
async fn stream_write(stream :&UnixStream, content :&[u8]) -> Result<(), Box<dyn Error>> {
    stream.writable().await?;
    loop {
        match stream.try_write(content) {
            Ok(_n) => {
            println!("sent {} bytes of data :\n {}", _n, String::from_utf8(content.to_vec()).unwrap());
                break;
            }
            Err(_e) => {
                continue;
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
