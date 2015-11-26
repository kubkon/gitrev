extern crate time;

use std::process::Command;
use std::error::Error;
use std::fmt;
use std::io::{Read, Write};
use std::fs::File;

use self::GitRevError::{Git, DateTime, FileIO};

/// Represents different types of errors that may arise when running `gitrev`.
#[derive(Debug)]
pub enum GitRevError {
    /// Invoking `git` command failed.
    Git(String),
    /// Parsing system time failed.
    DateTime(String),
    /// Reading from/writing to a file failed.
    FileIO(String),
}

impl GitRevError {
    /// Print this error and immediately exit program.
    pub fn exit(&self) -> ! {
        println!("{}", self);
        ::std::process::exit(1)
    }
}

impl fmt::Display for GitRevError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Git(ref s) => write!(f, "git error: {}", s),
            DateTime(ref s) => write!(f, "system time error: {}", s),
            FileIO(ref s) => write!(f, "file IO error: {}", s),
        }
    }
}

impl Error for GitRevError {
    fn description(&self) -> &str {
        match *self {
            Git(..) => "git command failed",
            DateTime(..) => "failed to parse system time",
            FileIO(..) => "file input-output error",
        }
    }
}

/// The main gitrev type which takes two arguments:
/// 1) the input filename that is the template, and
/// 2) the output filename that will be generated
#[derive(Clone, Debug)]
pub struct GitRev {
    template_filename: String,
    generated_filename: String,
}

impl GitRev {
    pub fn new(template_filename: &str, generated_filename: &str) -> GitRev {
        GitRev {
            template_filename: String::from(template_filename),
            generated_filename: String::from(generated_filename),
        }
    }

    /// Generates the output file with `SubWCRev.exe` keywords
    /// substituted for `git` command equivalents.
    ///
    /// The process consists of the following steps:
    /// 1) get all the necessary data from git
    /// 2) read in the contents of the template file
    /// 3) parse the contents and replace all special keywords with git data
    /// 4) write the contents to the output file
    pub fn generate_from_template(&self) -> Result<(), GitRevError> {
        // Get all the necessary data from git and system
        let curr_rev = try!(self.git_command("describe --always"));
        let curr_branch = try!(self.git_command("rev-parse --abbrev-ref HEAD"));
        let remote_url = try!(self.git_command("config --get remote.origin.url"));
        let build_time = try!(self.build_time());
        println!("Current revision: {}", curr_rev);
        println!("Build time: {}", build_time);
        println!("Current branch: {}", curr_branch);
        println!("Remote url: {}", remote_url);

        // Read in contents of the template file
        let mut contents = try!(self.read_in_template_file());

        // Parse the contents in search of special delimiters
        // and replace with the git data
        contents = contents.replace("$WCREV$", &curr_rev)
                           .replace("$WCNOW$", &build_time)
                           .replace("$WCURL$", &remote_url);

        // Write contents to the destination file
        self.write_to_dst_file(&contents)
    }

    fn git_command(&self, cmd: &str) -> Result<String, GitRevError> {
        // Parse args
        let args: Vec<&str> = cmd.split_whitespace().collect();
        let output = match Command::new("git").args(&args).output() {
            Err(e) => return Err(Git(String::from(e.description()))),
            Ok(out) => out,
        };
        if output.stdout.is_empty() {
            let err_msg = String::from_utf8_lossy(&output.stderr);
            return Err(Git(String::from(err_msg.trim())));
        }
        let result = String::from_utf8_lossy(&output.stdout);
        Ok(String::from(result.trim()))
    }

    fn build_time(&self) -> Result<String, GitRevError> {
        match time::strftime("%F %T %Z", &time::now_utc()) {
            Err(e) => Err(DateTime(String::from(e.description()))),
            Ok(out) => Ok(out),
        }
    }

    fn read_in_template_file(&self) -> Result<String, GitRevError> {
        let mut f = match File::open(&self.template_filename) {
            Err(e) => return Err(FileIO(String::from(e.description()))),
            Ok(f) => f,
        };
        let mut contents = String::new();
        match f.read_to_string(&mut contents) {
            Err(e) => return Err(FileIO(String::from(e.description()))),
            Ok(_) => (),
        };
        Ok(contents)
    }

    fn write_to_dst_file(&self, contents: &str) -> Result<(), GitRevError> {
        let mut f = match File::create(&self.generated_filename) {
            Err(e) => return Err(FileIO(String::from(e.description()))),
            Ok(f) => f,
        };
        match f.write_all(contents.as_bytes()) {
            Err(e) => Err(FileIO(String::from(e.description()))),
            Ok(()) => Ok(()),
        }
    }
}
