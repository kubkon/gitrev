#![crate_name="gitrev"]
extern crate time;

use std::process::Command;
use std::error::Error;

pub type GitRevResult = Result<String, String>;

pub fn git_describe() -> GitRevResult {
    let git_describe = match Command::new("git").arg("describe").arg("--always").output() {
        Err(e) => {
            let mut err = String::from("git-describe failed with error:\n");
            err.push_str(e.description());
            return Err(err);
        },
        Ok(out) => out
    };
    if git_describe.stdout.is_empty() {
        let mut err = String::from("git-describe failed with error:\n");
        err.push_str(&String::from_utf8_lossy(&git_describe.stderr));
        return Err(err);
    }
    let git_describe_str = String::from_utf8_lossy(&git_describe.stdout);
    Ok(String::from(git_describe_str.trim()))
}

pub fn git_branch() -> GitRevResult {
    let git_branch = match Command::new("git").arg("branch").output() {
        Err(e) => {
            let mut err = String::from("git-branch failed with error:\n");
            err.push_str(e.description());
            return Err(err);
        },
        Ok(out) => out
    };
    if git_branch.stdout.is_empty() {
        let mut err = String::from("git-branch failed with error:\n");
        err.push_str(&String::from_utf8_lossy(&git_branch.stderr));
        return Err(err);
    }
    let git_branch_str = String::from_utf8_lossy(&git_branch.stdout);
    Ok(String::from(git_branch_str.split_whitespace().nth(1).unwrap()))
}

pub fn git_remote_url() -> GitRevResult {
    let git_url = match Command::new("git")
                                .arg("config")
                                .arg("--get")
                                .arg("remote.origin.url")
                                .output() {
        Err(e) => {
            let mut err = String::from("git-remote-url failed with error:\n");
            err.push_str(e.description());
            return Err(err);
        },
        Ok(out) => out
    };
    if git_url.stdout.is_empty() {
        let mut err = String::from("git-remote-url failed with error:\n");
        err.push_str(&String::from_utf8_lossy(&git_url.stderr));
        return Err(err);
    }
    let git_url_str = String::from_utf8_lossy(&git_url.stdout);
    Ok(String::from(git_url_str.trim()))
}

pub fn build_time() -> GitRevResult {
    match time::strftime("%F %T %Z", &time::now_utc()) {
        Err(_) => Err(String::from("failed to get system time")),
        Ok(out) => Ok(out)
    }
}
