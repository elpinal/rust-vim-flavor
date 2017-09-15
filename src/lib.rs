//! A manager of Vim plugins.
#![warn(missing_docs)]
#![feature(ascii_ctype)]
#![feature(slice_patterns)]

mod parse;

use parse::{Parser, ParseError};

use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

fn get_root() -> Option<PathBuf> {
    env::home_dir().map(|mut p| {
        p.push(".vim");
        p.push("rflavors");
        p
    })
}

fn complete(s: &str) -> String {
    match *s.split('/').collect::<Vec<&str>>() {
        [vs] => format!("git://github.com/vim-scripts/{}.git", vs),
        [u, r] => format!("git://github.com/{}/{}.git", u, r),
        _ => s.to_owned(),
    }
}

fn is_valid(ch: char) -> bool {
    !ch.is_alphanumeric() && ch != '-' && ch != '_' && ch != '.'
}

/// Parses content of the flavor file and installs plugins which are described in it.
pub fn install(s: &str) -> Result<(), InstallError> {
    let root = get_root().ok_or(InstallError::GetHome)?;
    for f in Parser::new(s).parse()? {
        let r = complete(&f.repo);
        let n = f.repo.replace(is_valid, "_");
        let d = root.join(n);
        if d.exists() {
            continue;
        }
        let dest = d.to_str().expect(
            "failed to build destination path for 'git clone'",
        );
        let status = Command::new("git")
            .args(&["clone", "--depth", "1", &r, dest])
            .status()?;
        if !status.success() {
            return Err(InstallError::Exit(status));
        }
    }
    Ok(())
}

/// Parses content of the flavor file and updates plugins which are described in it.
pub fn update(s: &str) -> Result<(), InstallError> {
    let root = get_root().ok_or(InstallError::GetHome)?;
    for f in Parser::new(s).parse()? {
        let n = f.repo.replace(
            |ch: char| !ch.is_alphanumeric() && ch != '-' && ch != '_' && ch != '.',
            "_",
        );
        let d = root.join(n);
        if !d.exists() {
            continue;
        }
        let dest = d.to_str().expect(
            "failed to build destination path for 'git pull'",
        );
        let status = Command::new("git")
            .current_dir(dest)
            .args(&["pull"])
            .status()?;
        if !status.success() {
            return Err(InstallError::Exit(status));
        }
    }
    Ok(())
}

#[derive(Debug)]
/// Represents an error while installing plugins.
pub enum InstallError {
    /// Cannot get the home directory.
    GetHome,
    /// Error when executing the 'git' command.
    IO(io::Error),
    /// Given Flavor file cannot be parsed successfully.
    Parse(ParseError),
    /// Command exited with the exit status.
    Exit(ExitStatus),
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstallError::GetHome => write!(f, "error while getting home path"),
            InstallError::IO(ref e) => write!(f, "IO error: {}", e),
            InstallError::Parse(ref e) => write!(f, "parse error: {}", e),
            InstallError::Exit(status) => status.fmt(f),
        }
    }
}

impl Error for InstallError {
    fn description(&self) -> &str {
        match *self {
            InstallError::GetHome => "error while getting home path",
            InstallError::IO(ref e) => e.description(),
            InstallError::Parse(ref e) => e.description(),
            InstallError::Exit(_) => "command exited",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            InstallError::GetHome => None,
            InstallError::IO(ref e) => e.cause(),
            InstallError::Parse(ref e) => e.cause(),
            InstallError::Exit(_) => None,
        }
    }
}

impl From<ParseError> for InstallError {
    fn from(e: ParseError) -> InstallError {
        InstallError::Parse(e)
    }
}

impl From<io::Error> for InstallError {
    fn from(e: io::Error) -> InstallError {
        InstallError::IO(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete() {
        let s = "vspec";
        let rs = complete(s);
        assert_eq!(rs, "git://github.com/vim-scripts/vspec.git".to_owned());

        let s = "elpinal/vim-goyacc";
        let rs = complete(s);
        assert_eq!(rs, "git://github.com/elpinal/vim-goyacc.git".to_owned());

        let s = "https://github.com/elpinal/vim-goyacc";
        let rs = complete(s);
        assert_eq!(rs, "https://github.com/elpinal/vim-goyacc".to_owned());
    }
}
