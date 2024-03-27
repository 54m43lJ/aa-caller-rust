use std::{error::Error, fs, path::{Path, PathBuf}};
use tokio::net::{UnixListener, UnixStream};
use crate::{command::{get_logs, get_status, get_unconfined, profile_load, profile_set}, common::{AsyncHandler, Call, Handler, ProfStatus, ProfileOp}, LOG_FILES};

pub struct Server {
    pub call :Call
}

struct ProfileServer {
    op :ProfileOp
}

impl AsyncHandler for Server {
    async fn handle(&self) -> Result<(), Box<dyn Error>> {
        match &self.call {
            Call::None => {
                listen().await?;
            }
            Call::Daemon => { listen().await? }
            Call::Logs => {
                // let log = get_logs(&LOG_FILES);
            }
            Call::Status => {
                // let buf = get_status();
            }
            Call::Unconfined => {
                // let buf = get_unconfined();
            }
            Call::Profile(op) => {
                ProfileServer{ op :op.clone() }.handle()?;
            }
        }
        Ok(())
    }
}

impl Handler for ProfileServer {
    fn handle(&self) -> Result<(), Box<dyn Error>> {
        match &self.op {
            ProfileOp::Load(profile) => {
                let path = PathBuf::try_from(profile)?;
                profile_load(&path);
            }
            ProfileOp::Disable(profile) => {
                profile_set(profile, ProfStatus::Disabled);
            }
            ProfileOp::Status(profile, status) => {
                profile_set(profile, status.clone());
            }
        }
        Ok(())
    }
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
        Ok(0) => {
        }
        Ok(n) => {
            buf.truncate(n);
            let req = String::from_utf8(buf).unwrap();
            let output = match req.as_str() {
                "logs" => get_logs(&LOG_FILES),
                "status" => get_status(),
                "unconfined" => get_unconfined(),
                _ => { Vec::<u8>::new() }
            };
            stream_write(&stream, output).await?;
        }
        Err(_e) => {}
    }
    Ok(())
}

/// write any slice of byte string to the stream
async fn stream_write(stream :&UnixStream, content :Vec<u8>) -> Result<(), Box<dyn Error>> {
    stream.writable().await?;
    loop {
        match stream.try_write(&content[..]) {
            Ok(_n) => {
                // println!("sent {} bytes of data :\n {}", n, String::from_utf8(content)?);
                break;
            }
            Err(_e) => { continue; }
        }
    }
    Ok(())
}

async fn _debug_stream(stream: &UnixStream) -> Result<(), Box<dyn Error>> {
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
