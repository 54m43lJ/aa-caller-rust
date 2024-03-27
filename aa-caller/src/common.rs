use std::error::Error;

use clap::ValueEnum;

pub static LOG_FILES :[&str; 1] = ["/var/log/audit/audit.log"];

pub enum Call {
    None,
    Daemon,
    Logs,
    Status,
    Unconfined,
    Profile(ProfileOp)
}

#[derive(Clone)]
pub enum ProfileOp {
    Load(String),
    Disable(String),
    Status(String, ProfStatus)
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
