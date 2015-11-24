#![crate_name="gitrev"]
extern crate time;

use std::process::Command;

pub fn git_describe() -> Result<String, std::io::Error> {
    let git_describe = try!(Command::new("git").arg("describe").arg("--always").output());
    let git_describe_str = String::from_utf8_lossy(&git_describe.stdout);
    Ok(String::from(git_describe_str.trim()))
}

pub fn git_branch() -> Result<String, std::io::Error> {
    let git_branch = try!(Command::new("git").arg("branch").output());
    let git_branch_str = String::from_utf8_lossy(&git_branch.stdout);
    Ok(String::from(git_branch_str.split_whitespace().nth(1).unwrap()))
}

pub fn git_remote_url() -> Result<String, std::io::Error> {
    let git_url = try!(Command::new("git")
                               .arg("config")
                               .arg("--get")
                               .arg("remote.origin.url")
                               .output());
    let git_url_str = String::from_utf8_lossy(&git_url.stdout);
    Ok(String::from(git_url_str.trim()))
}

pub fn build_time() -> Result<String, time::ParseError> {
    time::strftime("%F %T %Z", &time::now_utc())
}
