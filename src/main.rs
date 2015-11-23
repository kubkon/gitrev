extern crate rustc_serialize;
extern crate docopt;

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
    let git_rev = Command::new("git")
                           .arg("describe")
                           .arg("--always")
                           .output()
                           .unwrap_or_else(|e| { panic!("Failed to execute process: {}", e) });
    println!("{}", String::from_utf8_lossy(&git_rev.stdout));
}
