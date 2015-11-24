extern crate rustc_serialize;
extern crate docopt;
extern crate gitrev;

use docopt::Docopt;

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
    let curr_rev = gitrev::git_describe()
                          .unwrap_or_else(|e| panic!(e));
    let curr_branch = gitrev::git_branch()
                             .unwrap_or_else(|e| panic!(e));
    let remote_url = gitrev::git_remote_url()
                            .unwrap_or_else(|e| panic!(e));
    let build_time = gitrev::build_time()
                            .unwrap_or_else(|e| panic!(e));
    println!("Current revision: {}", curr_rev);
    println!("Build time: {}", build_time);
    println!("Current branch: {}", curr_branch);
    println!("Remote url: {}", remote_url);
}
