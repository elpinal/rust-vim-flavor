//! A manager of Vim plugins.
#![warn(missing_docs)]
#![allow(unused)]
#![feature(ascii_ctype)]

mod parse;

use parse::Parser;

fn parse() {
    let mut p = Parser::new("flavor '1'");
    println!("{:?}", p.parse());
}
