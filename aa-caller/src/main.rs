use std::error::Error;
use caller::Caller;
use server::Server;
use clap::{Parser, Subcommand};
use common::{AsyncHandler, Call, Handler, ProfStatus, ProfileOp};

mod caller;
mod server;
mod command;
mod common;

#[derive(Parser)]
#[command(version,about)]
struct Opts {

// cli mode
    // profile operations
    #[arg(index=1, requires="prof_opts")]
    profile :Option<String>,
    #[arg(short='L', group="prof_opts")]
    prof_load :bool,
    #[arg(short='d', group="prof_opts")]
    prof_disable:bool,
    #[arg(short='t', value_enum, group="prof_opts")]
    prof_status :Option<ProfStatus>,

    // common options
    #[arg(short, exclusive=true, help="print all available kernel logs")]
    // logs :Option<Vec<PathBuf>>,
    logs :bool,
    #[arg(short, exclusive=true, help="get_status")]
    status :bool,
    #[arg(short, exclusive=true, help="get_unconfined")]
    unconfined :bool,

    #[command(subcommand)]
    action :Option<Call>
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
            Call::Daemon => {
                server.call = Call::Daemon;
            }
            Call::Logs => {
                server.call = Call::Logs;
            }
        };
        server.handle().await?;
    // let Caller handle it
    } else {
        // does nothing by default
        let mut caller = Caller{ call :Call::None };
        if opts.logs {
            caller.call = Call::Logs;
        } else if opts.status {
            caller.call = Call::Status;
        } else if opts.unconfined {
            caller.call = Call::Unconfined;
        } else if let Some(profile) = opts.profile {
            if opts.prof_load {
                caller.call = Call::Profile(ProfileOp::Load);
            } else if opts.prof_disable {
                caller.call = Call::Profile(ProfileOp::Disable);
            } else if let Some(status) = opts.prof_status {
                caller.call = Call::Profile();
            }
        // or just listen
        } else {
            // listen().await?;
        }
        caller.handle()?;
    }
    Ok(())
}
