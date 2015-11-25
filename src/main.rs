extern crate rustc_serialize;
extern crate docopt;
extern crate gitrev;

use docopt::Docopt;
use gitrev::GitRev;

const USAGE: &'static str = "
GITREV is a utility program that is meant to serve as a replacement for
SubWCRev.exe.

Usage:
    gitrev.exe <template-filename> <generated-filename>
    gitrev.exe (-h | --help)
    gitrev.exe --version

Options:
    -h --help   Prints this message.
    --version   Prints version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_template_filename: String,
    arg_generated_filename: String,
}

fn main() {
    // Parse command line arguments
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    let gr = GitRev::new(&args.arg_template_filename, &args.arg_generated_filename);
    gr.generate_from_template().unwrap_or_else(|e| e.exit());
}
