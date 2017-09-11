//! A manager of Vim plugins.
#![warn(missing_docs)]
#![allow(unused)]
#![feature(ascii_ctype)]
#![feature(slice_patterns)]

mod parse;

use parse::{Parser, ParseError};

use std::process::Command;
use std::env;
use std::io;
use std::path::PathBuf;

fn get_root() -> Option<PathBuf> {
    let p = env::home_dir();
    if p.is_none() {
        return None;
    }
    let mut p = p.unwrap();
    p.push(".rust-vim-flavor");
    p.push("repos");
    Some(p)
}

fn parse(s: &str) -> Result<Vec<String>, ParseError> {
    let mut p = Parser::new(s);
    let fs = p.parse()?;
    let rs = fs.iter().map(|f| match *f.repo
        .split('/')
        .collect::<Vec<&str>>() {
        [vs] => format!("git://github.com/vim-scripts/{}.git", vs),
        [u, r] => format!("git://github.com/{}/{}.git", u, r),
        _ => f.repo.to_owned(),
    });
    Ok(rs.collect())
}

fn install(s: &str) -> Result<(), InstallError> {
    let root = get_root().ok_or(InstallError::GetHome)?;
    for r in parse(s)? {
        let n = r.split('/').last().unwrap();
        Command::new("git")
            .args(&["--depth", "1", &r, root.join(n).to_str().unwrap()])
            .spawn()?;
    }
    Ok(())
}

enum InstallError {
    GetHome,
    Git(io::Error),
    Parse(ParseError),
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
    fn test_parse() {
        let s = "flavor 'vspec'";
        let rs = parse(s);
        assert_eq!(
            rs,
            Ok(vec!["git://github.com/vim-scripts/vspec.git".to_owned()])
        );

        let s = "flavor 'elpinal/vim-goyacc'";
        let rs = parse(s);
        assert_eq!(
            rs,
            Ok(vec!["git://github.com/elpinal/vim-goyacc.git".to_owned()])
        );

        let s = "flavor 'https://github.com/elpinal/vim-goyacc'";
        let rs = parse(s);
        assert_eq!(
            rs,
            Ok(vec!["https://github.com/elpinal/vim-goyacc".to_owned()])
        );
    }
}
