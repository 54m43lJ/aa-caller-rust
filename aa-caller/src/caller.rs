use std::{error::Error, fs::{self, File}, io::Write, path::PathBuf, process::Command};
use crate::common::{Call, Handler, ProfStatus, ProfileOp, LOG_FILES};

pub struct Caller {
    pub call :Call
}

struct ProfileCaller {
    op : ProfileOp
}

impl Handler for Caller {
    fn handle(&self) -> Result<(), Box<dyn Error>> {
        match &self.call {
            Call::None => {}
            Call::Daemon => { eprintln!("How do you even get here?"); }
            Call::Logs => {
                let log = get_logs(&LOG_FILES)?;
                println!("{}", String::from_utf8(log)?);
            }
            Call::Status => {}
            Call::Unconfined => {}
            Call::Profile(op) => {
                ProfileCaller{ op :op.clone() }.handle()?;
            }
        }
        Ok(())
    }
}

impl Handler for ProfileCaller {
    fn handle(&self) -> Result<(), Box<dyn Error>> {
        match &self.op {
            ProfileOp::Load(profile) => {
                let path = PathBuf::try_from(profile)?;
                let buf = fs::read(&path)?;
                let mut target = PathBuf::from("/etc/apparmor.d/");
                target.push(path.file_name().unwrap());
                let mut ftarget = File::create(&target)?;
                ftarget.write_all(&buf[..])?;
                Command::new("apparmor_parser")
                    .args(["-r", target.to_str().unwrap()])
                    .output().expect("Command failed!");
            }
            ProfileOp::Disable(profile) => {
                Command::new("aa-disable")
                    .arg(profile)
                    .output().expect("Command failed!");
            }
            ProfileOp::Status(profile, status) => {
                match status {
                    ProfStatus::Audit => {
                        Command::new("aa-audit")
                            .arg(profile)
                            .output().expect("Command failed!");
                    }
                    ProfStatus::Complain => {
                        Command::new("aa-complain")
                            .arg(profile)
                            .output().expect("Command failed!");
                    }
                    ProfStatus::Disabled => {
                        Command::new("aa-disable")
                            .arg(profile)
                            .output().expect("Command failed!");
                    }
                    ProfStatus::Enforce => {
                        Command::new("aa-enforce")
                            .arg(profile)
                            .output().expect("Command failed!");
                    }
                }
            }
        }
        Ok(())
    }
}

/// fetch logs from a kernel log file
fn get_logs(files :&[&str]) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut output :Vec<u8> = Vec::new();
    for f in files {
        let mut buf = fs::read(f)?;
        output.append(&mut buf);
    }
    Ok(output)
}
