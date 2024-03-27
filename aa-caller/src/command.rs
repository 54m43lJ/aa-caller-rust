/// Where all interfacing with fs & other programs are gracefully handled, no error should
/// propagate through here
/// For now: always send an empty response on any error, log the failure

use std::{fs::{self, File}, io::Write, path::PathBuf, process::Command};

use crate::common::ProfStatus;

pub fn get_logs(files :&[&str]) -> Vec<u8> {
    let mut output :Vec<u8> = Vec::new();
    for f in files {
        match fs::read(f) {
            Ok(mut buf) => {
                output.append(&mut buf);
            }
            Err(_) => {}
        }
    }
    output
}

pub fn get_status() -> Vec<u8> {
    match Command::new("aa-status")
        .arg("--json")
        .output() {
            Ok(o) => {
                if o.status.success() {
                    o.stdout
                } else {
                    b"{\"processes\": {}, \"profiles\": {}}".to_vec()
                }
            }
            Err(_) => {
                b"{\"processes\": {}, \"profiles\": {}}".to_vec()
            }
        }
}

pub fn get_unconfined() -> Vec<u8> {
    match Command::new("ps")
        .args(["-A", "--format", "pid,ppid,user,context,comm", "--no-header"])
        .output() {
            Ok(o) => {
                if o.status.success() {
                    o.stdout
                } else {
                    b"".to_vec()
                }
            }
            Err(_) => {
                b"".to_vec()
            }
        }
}

pub fn profile_load(path :&PathBuf) {
    let mut output = Vec::<u8>::new();
    match fs::read(&path) {
        Ok(mut buf) => { output.append(&mut buf); }
        Err(_) => { return; }
    }
    let mut target = PathBuf::from("/etc/apparmor.d/");
    target.push(path.file_name().unwrap());
    let mut ftarget = match File::create(&target) {
        Ok(f) => { f }
        Err(_) => { return; }
    };
    match ftarget.write_all(&output[..]) {
        Ok(_) => {}
        Err(_) => { return; }
    }
    match Command::new("apparmor_parser")
        .args(["-r", target.to_str().unwrap()])
        .output() {
            Ok(_) => {}
            Err(_) => { return; }
        }
}

pub fn profile_set(profile :&String, status :ProfStatus) {
    match status {
        ProfStatus::Audit => {
            match Command::new("aa-audit")
                .arg(profile)
                .output() {
                    _ => {}
                }
        }
        ProfStatus::Enforce => {
            match Command::new("aa-enforce")
                .arg(profile)
                .output() {
                    _ => {}
            }
        }
        ProfStatus::Complain => {
            match Command::new("aa-complain")
                .arg(profile)
                .output() {
                    _ => {}
            }
        }
        ProfStatus::Disabled => {
            match Command::new("aa-disable")
                .arg(profile)
                .output() {
                    _ => {}
            }
        }
    }
}
