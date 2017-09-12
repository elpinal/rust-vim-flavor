extern crate vim_flavor;

use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let r = run();
    if let Some(e) = r.err() {
        println!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), CLIError> {
    let name = env::args().nth(1).ok_or(CLIError::MissingArgument)?;
    let mut f = File::open(name)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    vim_flavor::install(&buffer)?;
    Ok(())
}

enum CLIError {
    MissingArgument,
    FlavorFile(io::Error),
    Install(vim_flavor::InstallError),
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CLIError::MissingArgument => write!(f, "1 argument needed"),
            CLIError::FlavorFile(ref e) => write!(f, "io error: {}", e),
            CLIError::Install(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for CLIError {
    fn from(e: io::Error) -> CLIError {
        CLIError::FlavorFile(e)
    }
}

impl From<vim_flavor::InstallError> for CLIError {
    fn from(e: vim_flavor::InstallError) -> CLIError {
        CLIError::Install(e)
    }
}
