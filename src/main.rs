extern crate rustc_serialize;
extern crate docopt;
extern crate gitrev;

use docopt::Docopt;
use std::io::{Read, Write};
use std::fs::File;

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

    // Get all the necessary data from git and system
    let curr_rev = gitrev::git_describe().unwrap_or_else(|e| {
        println!("{}", e);
        ::std::process::exit(1);
    });
    let curr_branch = gitrev::git_branch().unwrap_or_else(|e| {
        println!("{}", e);
        ::std::process::exit(1);
    });
    let remote_url = gitrev::git_remote_url().unwrap_or_else(|e| {
        println!("{}", e);
        ::std::process::exit(1);
    });
    let build_time = gitrev::build_time().unwrap_or_else(|e| {
        println!("{}", e);
        ::std::process::exit(1);
    });
    println!("Current revision: {}", curr_rev);
    println!("Build time: {}", build_time);
    println!("Current branch: {}", curr_branch);
    println!("Remote url: {}", remote_url);

    // Read in contents of the template file
    let mut f = File::open(&args.arg_src_version_file).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    // Parse the contents in search of special delimiters
    // and replace with the git data
    contents = contents.replace("$WCREV$", &curr_rev);
    contents = contents.replace("$WCNOW$", &build_time);
    contents = contents.replace("$WCURL$", &remote_url);

    // Write contents to the destination file
    f = File::create(&args.arg_dst_version_file).unwrap();
    f.write_all(&contents.into_bytes()).unwrap();
}
