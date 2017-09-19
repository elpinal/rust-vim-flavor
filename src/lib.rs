//! A manager of Vim plugins.
#![warn(missing_docs)]
#![feature(ascii_ctype)]
#![feature(slice_patterns)]

mod parse;
mod version;

pub use parse::{Flavor, Parser, ParseError};

use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};

/// Gets the default root directory.
/// When succeeded in obtaining the home direcotry, returns `$HOME/.vim/rflavors`.
/// Otherwise, returns None.
pub fn get_root() -> Option<PathBuf> {
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

fn is_invalid(ch: char) -> bool {
    !ch.is_alphanumeric() && ch != '-' && ch != '_' && ch != '.'
}

/// Parses content of the flavor file and installs plugins which are described in it.
pub fn install(fs: &[Flavor], root: &Path) -> Result<(), InstallError> {
    for f in fs {
        let n = f.repo.replace(is_invalid, "_");
        let d = root.join(n);
        if d.exists() {
            continue;
        }
        let dest = d.to_str().expect(
            "failed to build destination path for 'git clone'",
        );
        let r = complete(&f.repo);
        let output = Command::new("git")
            .args(&["clone", "--depth", "1", "--branch", &f.branch, &r, dest])
            .output()?;
        if !output.status.success() {
            eprintln!(
                "{}: failed to install:\n\
                 {}",
                f.repo,
                String::from_utf8_lossy(&output.stderr)
            );
            return Err(InstallError::Exit(output.status));
        }
    }
    Ok(())
}

/// Parses content of the flavor file and updates plugins which are described in it.
pub fn update(fs: &[Flavor], root: &Path) -> Result<(), InstallError> {
    git_with_flavor(fs, root, false, |f, _| vec!["pull", "origin", &f.branch])
}

fn git_with_flavor<'a, 'b>(
    fs: &'a [Flavor],
    root: &Path,
    not: bool,
    args: fn(&'a Flavor, &str) -> Vec<&'b str>,
) -> Result<(), InstallError> {
    for f in fs {
        let n = f.repo.replace(is_invalid, "_");
        let d = root.join(n);
        if not == d.exists() {
            eprintln!("Skipped {}: not installed yet.", f.repo);
            continue;
        }
        let dest = d.to_str().expect(
            "failed to build destination path for 'git pull'",
        );
        let output = Command::new("git")
            .current_dir(dest)
            .args(args(f, dest))
            .output()?;
        if !output.status.success() {
            eprintln!("{}:", f.repo);
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            return Err(InstallError::Exit(output.status));
        }
    }
    Ok(())
}
#[derive(Debug)]
/// Represents an error while installing plugins.
pub enum InstallError {
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
            InstallError::IO(ref e) => write!(f, "IO error: {}", e),
            InstallError::Parse(ref e) => write!(f, "parse error: {}", e),
            InstallError::Exit(status) => status.fmt(f),
        }
    }
}

impl Error for InstallError {
    fn description(&self) -> &str {
        match *self {
            InstallError::IO(ref e) => e.description(),
            InstallError::Parse(ref e) => e.description(),
            InstallError::Exit(_) => "command exited",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
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
    use std::fs::remove_dir_all;

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
        assert_eq!(rs, s.to_owned());
    }

    #[test]
    fn test_is_invalid() {
        assert!(!is_invalid('a'));
        assert!(!is_invalid('1'));
        assert!(!is_invalid('.'));
        assert!(!is_invalid('-'));
        assert!(!is_invalid('_'));

        assert!(is_invalid('!'));
        assert!(is_invalid('/'));
        assert!(is_invalid('~'));
        assert!(is_invalid(' '));
        assert!(is_invalid(','));
    }

    #[test]
    fn test_install() {
        let mut dir = env::temp_dir();
        dir.push("rust-vim-flavor-install-test");

        let r = install(&[Flavor::new("vspec")], &dir);
        assert!(dir.join("vspec").join(".git").exists());
        assert!(r.is_ok());

        let r = install(&[Flavor::new("no/such/vim/plugin")], &dir);
        if let Some(e) = remove_dir_all(dir).err() {
            eprintln!("cannot remove a temporary directory: {}", e);
        }
        assert!(r.is_err());
    }

    #[test]
    fn test_update() {
        let mut dir = env::temp_dir();
        dir.push("rust-vim-flavor-update-test");

        let r = install(&[Flavor::new("vspec")], &dir);
        assert!(r.is_ok());

        let r = update(&[Flavor::new("vspec")], &dir);
        assert!(dir.join("vspec").join(".git").exists());
        if let Some(e) = remove_dir_all(dir).err() {
            eprintln!("cannot remove a temporary directory: {}", e);
        }
        assert!(r.is_ok());
    }
}
