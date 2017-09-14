extern crate vim_flavor;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

fn main() {
    std::process::exit(run().unwrap_or_else(|e| {
        writeln!(io::stderr(), "{}", e).unwrap();
        1
    }))
}

fn run() -> Result<i32, CLIError> {
    let mut args = env::args();
    Ok(match args.nth(1) {
        None => {
            writeln!(io::stderr(), "{}", HELP_MESSAGE).unwrap();
            2
        }
        Some(cmd) => {
            with_cmd(&cmd, args)?;
            0
        }
    })
}

fn with_cmd(cmd: &str, args: env::Args) -> Result<(), CLIError> {
    match cmd {
        "help" => help(args),
        "install" => install(),
        cmd => no_cmd(cmd),
    }
}

fn no_cmd(cmd: &str) -> Result<(), CLIError> {
    Err(CLIError::NoCommand(cmd.to_owned()))
}

const HELP_MESSAGE: &'static str = "\
Rust-vim-flavor is a tool to manage Vim plugins.

Usage:

        vim-flavor command [arguments]

Commands:

        help    show this help
        install install Vim plugins according to VimFlavor file
";

fn help(mut args: env::Args) -> Result<(), CLIError> {
    match args.next() {
        Some(name) => Err(CLIError::NoCommand(name))?,
        None => println!("{}", HELP_MESSAGE),
    }
    Ok(())
}

fn install() -> Result<(), CLIError> {
    let name = "VimFlavor";
    let mut f = File::open(name)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    vim_flavor::install(&buffer)?;
    Ok(())
}

#[derive(Debug)]
enum CLIError {
    MissingArgument,
    FlavorFile(io::Error),
    Install(vim_flavor::InstallError),
    NoCommand(String),
}

impl fmt::Display for CLIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CLIError::MissingArgument => write!(f, "1 argument needed"),
            CLIError::FlavorFile(ref e) => write!(f, "IO error: {}", e),
            CLIError::Install(ref e) => write!(f, "{}", e),
            CLIError::NoCommand(ref name) => write!(f, "no such command: {}", name),
        }
    }
}

impl Error for CLIError {
    fn description(&self) -> &str {
        match *self {
            CLIError::MissingArgument => "not enough arguments",
            CLIError::FlavorFile(ref e) => e.description(),
            CLIError::Install(ref e) => e.description(),
            CLIError::NoCommand(_) => "no such command",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CLIError::MissingArgument => None,
            CLIError::FlavorFile(ref e) => e.cause(),
            CLIError::Install(ref e) => e.cause(),
            CLIError::NoCommand(_) => None,
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
