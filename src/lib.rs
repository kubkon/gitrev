#![crate_name="gitrev"]
extern crate time;

use std::process::Command;
use std::error::Error;
use std::fmt;

use self::GitRevError::{Git, DateTime};

pub type GitRevResult = Result<String, String>;

#[derive(Debug)]
pub enum GitRevError {
    Git(String),
    DateTime(String),
}

impl GitRevError {
    pub fn exit(&self) -> ! {
        println!("{}", self);
        ::std::process::exit(1)
    }
}

impl fmt::Display for GitRevError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Git(ref s) | DateTime(ref s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl Error for GitRevError {
    fn description(&self) -> &str {
        match *self {
            Git(..) => "git command failed",
            DateTime(..) => "failed to parse system time",
        }
    }
}

pub fn git_command(cmd: &str) -> Result<String, GitRevError> {
    // Parse args
    let args: Vec<&str> = cmd.split_whitespace().collect();
    let output = match Command::new("git").args(&args).output() {
        Err(e) => return Err(Git(String::from(e.description()))),
        Ok(out) => out
    };
    if output.stdout.is_empty() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(Git(String::from(err_msg.trim())));
    }
    let result = String::from_utf8_lossy(&output.stdout);
    Ok(String::from(result.trim()))
}

pub fn build_time() -> GitRevResult {
    match time::strftime("%F %T %Z", &time::now_utc()) {
        Err(_) => Err(String::from("failed to get system time")),
        Ok(out) => Ok(out)
    }
}
