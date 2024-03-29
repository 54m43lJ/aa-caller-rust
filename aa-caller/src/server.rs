use std::{error::Error, fs, io::Cursor, path::{Path, PathBuf}};
use tokio::net::{UnixListener, UnixStream};
use crate::{command::{get_logs, get_status, get_unconfined, profile_load, profile_set}, common::{AsyncHandler, Call, Handler, ProfStatus, ProfileArgs, ProfileOp}, LOG_FILES};
use prost::Message;
use self::protos::{profile, request};

pub mod protos {
    pub mod request {
        include!(concat!(env!("OUT_DIR"), "/protos.request.rs"));
    }
    pub mod profile {
        include!(concat!(env!("OUT_DIR"), "/protos.profile.rs"));
    }
}

pub struct Server {
    pub call :Call
}

struct ProfileServer {
    args :ProfileArgs
}

impl AsyncHandler for Server {
    async fn handle(self) -> Result<(), Box<dyn Error>> {
        match self.call {
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
            Call::Profile(args) => {
                ProfileServer{ args }.handle()?;
            }
        }
        Ok(())
    }
}

impl Handler for ProfileServer {
    fn handle(self) -> Result<(), Box<dyn Error>> {
        match self.args {
            ProfileArgs { profile, op: ProfileOp::Load, status: None } => {
                let path = PathBuf::try_from(profile)?;
                profile_load(&path);
            }
            ProfileArgs { profile, op: ProfileOp::Disable, status: None } => {
                profile_set(&profile, ProfStatus::Disabled);
            }
            ProfileArgs { profile, op: ProfileOp::Load, status: Some(status) } => {
                profile_set(&profile, status);
            }
            _ => {
                // malformed request
                eprintln!("How do you even get here?");
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

/// translate request into function calls
async fn process(stream: UnixStream) -> Result<(), Box<dyn Error>> {
    let req = request::Request::decode(
        &mut Cursor::new(stream_read(&stream).await?))?;
    let output = match request::Call::try_from(req.call) {
        Ok(request::Call::Daemon) => get_logs(&LOG_FILES),
        Ok(request::Call::Logs) => get_logs(&LOG_FILES),
        Ok(request::Call::Status) => get_status(),
        Ok(request::Call::Unconfined) => get_unconfined(),
        Ok(request::Call::Profile) => {
            let req_p = profile::ProfileReq::decode(
                &mut Cursor::new(stream_read(&stream).await?)
            )?;
            match profile::ProfileOp::try_from(req_p.op) {
                Ok(profile::ProfileOp::Load) => {

                }
                Ok(profile::ProfileOp::Status) => {}
                Ok(profile::ProfileOp::Disable) => {}
                Err(_) => {}
            };
            Vec::<u8>::new()
        }
        Err(_) => { Vec::<u8>::new() }
    };
    stream_write(&stream, output).await?;
    Ok(())
}

/// read everything from stream & compile together
async fn stream_read(stream :&UnixStream) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output = Vec::<u8>::new();
    let mut buf = vec![0; 2048];
    loop {
        stream.readable().await?;
        match stream.try_read(&mut buf) {
            Ok(0) => { break; }
            Ok(n) => {
                buf.truncate(n);
                output.append(&mut buf);
            }
            Err(_) => { continue; }
        }
    }
    Ok(output)
}

/// write any slice of byte string to the stream
async fn stream_write(stream :&UnixStream, content :Vec<u8>) -> Result<(), Box<dyn Error>> {
    loop {
        stream.writable().await?;
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
