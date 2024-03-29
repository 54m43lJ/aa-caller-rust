use std::error::Error;

use clap::{Args, Subcommand, ValueEnum};

pub static LOG_FILES :[&str; 1] = ["/var/log/audit/audit.log"];

#[derive(Clone, Subcommand)]
pub enum Call {
    None,
    Daemon,
    Logs,
    Status,
    Unconfined,
    Profile(ProfileArgs)
}

#[derive(Clone, Args)]
pub struct ProfileArgs {
    #[arg(index=1)]
    pub profile :String,
    #[command(subcommand)]
    pub op :ProfileOp,
    #[arg(short='t')]
    pub status :Option<ProfStatus>
}

#[derive(Clone, Subcommand)]
pub enum ProfileOp {
    Load,
    Disable,
    Status
}

#[derive(Clone, ValueEnum)]
pub enum ProfStatus {
    Enforce,
    Complain,
    Audit,
    Disabled
}

pub trait Handler {
    fn handle(self) -> Result<(), Box<dyn Error>>;
}

pub trait AsyncHandler {
    async fn handle(self) -> Result<(), Box<dyn Error>>;
}
