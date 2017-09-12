//! A manager of Vim plugins.
#![warn(missing_docs)]
#![allow(unused)]
#![feature(ascii_ctype)]
#![feature(slice_patterns)]

mod parse;

use parse::{Parser, ParseError, Flavor};

use std::env;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

fn get_root() -> Option<PathBuf> {
    let p = env::home_dir();
    if p.is_none() {
        return None;
    }
    let mut p = p.unwrap();
    p.push(".vim");
    p.push("rflavors");
    Some(p)
}

fn parse(s: &str) -> Result<Vec<Flavor>, ParseError> {
    let mut p = Parser::new(s);
    p.parse()
}

fn complete(s: &str) -> String {
    match *s.split('/').collect::<Vec<&str>>() {
        [vs] => format!("git://github.com/vim-scripts/{}.git", vs),
        [u, r] => format!("git://github.com/{}/{}.git", u, r),
        _ => s.to_owned(),
    }
}

/// Parses content of the flavor file and installs plugins which are described in it.
pub fn install(s: &str) -> Result<(), InstallError> {
    let root = get_root().ok_or(InstallError::GetHome)?;
    for f in parse(s)? {
        let r = complete(&f.repo);
        let n = f.repo.replace(
            |ch: char| !ch.is_alphanumeric() && ch != '-' && ch != '_' && ch != '.',
            "_",
        );
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

#[derive(Debug)]
/// Represents an error while installing plugins.
pub enum InstallError {
    /// Cannot get the home directory.
    GetHome,
    /// Error when executing the 'git' command.
    Git(io::Error),
    /// Given Flavor file cannot be parsed successfully.
    Parse(ParseError),
    /// Command exited with the exit status.
    Exit(ExitStatus),
}

impl fmt::Display for InstallError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InstallError::GetHome => write!(f, "error while getting home path"),
            InstallError::Git(ref e) => write!(f, "git failed: {}", e),
            InstallError::Parse(ref e) => write!(f, "parse error: {}", e),
            InstallError::Exit(status) => status.fmt(f),
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
        InstallError::Git(e)
    }
}

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
