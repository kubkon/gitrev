extern crate rustc_serialize;
extern crate docopt;
extern crate time;

use docopt::Docopt;
use std::process::Command;

const USAGE: &'static str = "
GITREV is a utility program that is meant to serve as a replacement for
SubWCRev.exe.

Usage:
    gitrev.exe <src-version-file> <dst-version-file>
    gitrev.exe (-h | --help)
    gitrev.exe --version

Options:
    -h --help   Prints this message.
    --version   Prints version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_src_version_file: String,
    arg_dst_version_file: String,
}

fn main() {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let git_describe = Command::new("git")
                               .arg("describe")
                               .arg("--always")
                               .output()
                               .unwrap_or_else(|e| { panic!("Failed to execute process: {}", e) });
    let git_describe_str = String::from_utf8_lossy(&git_describe.stdout);
    let curr_rev = &git_describe_str.trim();
    let time_now = time::strftime("%F %T %Z", &time::now_utc())
                        .unwrap_or_else(|e| { panic!("Failed to parse current time: {}", e) });
    let git_branch = Command::new("git")
                             .arg("branch")
                             .output()
                             .unwrap_or_else(|e| { panic!("Failed to execute process: {}", e) });
    let git_branch_str = String::from_utf8_lossy(&git_branch.stdout);
    let curr_branch = &git_branch_str.split_whitespace().nth(1).unwrap();
    let git_url = Command::new("git")
                          .arg("config")
                          .arg("--get")
                          .arg("remote.origin.url")
                          .output()
                          .unwrap_or_else(|e| { panic!("Failed to execute the process: {}", e) });
    let git_url_str = String::from_utf8_lossy(&git_url.stdout);
    let curr_url = &git_url_str.trim();
    println!("Current revision: {}", curr_rev);
    println!("Current time: {}", time_now);
    println!("Current branch: {}", curr_branch);
    println!("Current url: {}", curr_url);
}
