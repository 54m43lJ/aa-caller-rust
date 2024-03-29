use std::{error::Error, path::PathBuf};
use crate::{command::{get_logs, get_status, get_unconfined, profile_load, profile_set}, common::{Call, Handler, ProfStatus, ProfileArgs, ProfileOp, LOG_FILES}};

pub struct Caller {
    pub call :Call
}

struct ProfileCaller {
    args :ProfileArgs
}

impl Handler for Caller {
    fn handle(self) -> Result<(), Box<dyn Error>> {
        match self.call {
            Call::None => {}
            Call::Daemon => { eprintln!("How do you even get here?"); }
            Call::Logs => {
                let log = get_logs(&LOG_FILES);
                println!("{}", String::from_utf8(log)?);
            }
            Call::Status => {
                let buf = get_status();
                println!("{}", String::from_utf8(buf)?);
            }
            Call::Unconfined => {
                let buf = get_unconfined();
                println!("{}", String::from_utf8(buf)?);
            }
            Call::Profile(args) => {
                ProfileCaller{ args }.handle()?;
            }
        }
        Ok(())
    }
}

impl Handler for ProfileCaller {
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
