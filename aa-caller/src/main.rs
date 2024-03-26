use std::{error::Error, fs, path::Path, u8};
use caller::Caller;
use server::Server;
use tokio::net::{UnixListener, UnixStream};
use clap::{Parser, Subcommand};
use common::{Call, Handler, ProfStatus, ProfileOp};

mod caller;
mod server;
mod common;

#[derive(Parser)]
#[command(version,about)]
struct Opts {

    /// profile operations
    #[arg(index=1, requires="prof_opts")]
    profile :Option<String>,
    #[arg(short='L', group="prof_opts")]
    prof_load :bool,
    #[arg(short='d', group="prof_opts")]
    prof_disable:bool,
    #[arg(short='t', value_enum, group="prof_opts")]
    prof_status :Option<ProfStatus>,

    /// common options
    #[arg(short, exclusive=true, help="print all available kernel logs")]
    // logs :Option<Vec<PathBuf>>,
    logs :bool,
    #[arg(short, exclusive=true, help="get_status")]
    status :bool,
    #[arg(short, exclusive=true, help="get_unconfined")]
    unconfined :bool,

    #[command(subcommand)]
    action :Option<Action>
}

#[derive(Subcommand)]
enum Action {
    Daemon,
    Logs
}

static LOG_FILES :[&str; 1] = ["/var/log/audit/audit.log"];

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = Opts::parse();
    // let Server handle it
    if let Some(action) = opts.action {
        // does nothing by default
        let mut server = Server{ call :Call::None};
        match action {
            Action::Daemon => {
                server.call = Call::Daemon;
            }
            Action::Logs => {
                server.call = Call::Logs;
            }
        };
        server.handle()?;
    // let Caller handle it
    } else {
        // does nothing by default
        let mut caller = Caller{ call :Call::None };
        if opts.logs {
            caller.call = Call::Logs;
        } else if let Some(profile) = opts.profile {
            if opts.prof_load {
                caller.call = Call::Profile(ProfileOp::Load(profile));
            } else if opts.prof_disable {
                caller.call = Call::Profile(ProfileOp::Disable(profile));
            } else if let Some(status) = opts.prof_status {
                match status {
                    ProfStatus::Audit => {
                        caller.call = Call::Profile(ProfileOp::Status(profile, ProfStatus::Audit));
                    }
                    ProfStatus::Complain => {
                        caller.call = Call::Profile(ProfileOp::Status(profile, ProfStatus::Complain));
                    }
                    ProfStatus::Disabled => {
                        caller.call = Call::Profile(ProfileOp::Status(profile, ProfStatus::Disabled));
                    }
                    ProfStatus::Enforce => {
                        caller.call = Call::Profile(ProfileOp::Status(profile, ProfStatus::Enforce));
                    }
                }
            }
        // or just listen
        } else {
            listen().await?;
        }
        caller.handle()?;
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
                    match get_logs(&LOG_FILES) {
                        Ok(log) => { 
                            stream_write(stream, &log[..]).await?;
                            }
                        Err(_e) => { stream_write(stream, "FAILED".as_bytes()).await? }
                    };
                }
                _ => {}
            }
        }
        Err(_e) => {}
    }
    Ok(())
}

/// fetch logs from a kernel log file
fn get_logs(files :&[&str]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output :Vec<u8> = Vec::new();
    for f in files {
        // let path = PathBuf::try_from(f)?;
        // let mut buf = read_from_file(path)?;
        let mut buf = fs::read(f)?;
        output.append(&mut buf);
    }
    Ok(output)
}

/// write any slice of byte string to the stream
async fn stream_write(stream :&UnixStream, content :&[u8]) -> Result<(), Box<dyn Error>> {
    stream.writable().await?;
    loop {
        match stream.try_write(content) {
            Ok(n) => {
            println!("sent {} bytes of data :\n {}", n, String::from_utf8(content.to_vec()).unwrap());
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
