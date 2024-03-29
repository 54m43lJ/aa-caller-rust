use std::error::Error;

use clap::{Args, Subcommand, ValueEnum};

use crate::command;

pub static LOG_FILES :[&str; 1] = ["/var/log/audit/audit.log"];

#[derive(Subcommand)]
pub enum Call {
    None,
    Daemon,
    Logs,
    Status,
    Unconfined,
    Profile(ProfileArgs)
}

#[derive(Args)]
struct ProfileArgs {
    #[arg(index=1)]
    profile :String,
    #[command(subcommand)]
    op :ProfileOp,
    #[arg(short)]
    status :ProfStatus
}

#[derive(Subcommand)]
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
    fn handle(&self) -> Result<(), Box<dyn Error>>;
}

pub trait AsyncHandler {
    async fn handle(&self) -> Result<(), Box<dyn Error>>;
}
