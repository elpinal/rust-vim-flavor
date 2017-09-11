//! A manager of Vim plugins.
#![warn(missing_docs)]
#![allow(unused)]
#![feature(ascii_ctype)]
#![feature(slice_patterns)]

mod parse;

use parse::{Parser, ParseError};

fn parse(s: &str) -> Result<Vec<String>, ParseError> {
    let mut p = Parser::new(s);
    let fs = p.parse()?;
    let rs = fs.iter().map(|f| match *f.repo
        .split('/')
        .collect::<Vec<&str>>() {
        [vs] => format!("git://github.com/vim-scripts/{}.git", vs),
        _ => f.repo.to_owned(),
    });
    Ok(rs.collect())
}
