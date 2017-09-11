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
        [u, r] => format!("git://github.com/{}/{}.git", u, r),
        _ => f.repo.to_owned(),
    });
    Ok(rs.collect())
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
        assert_eq!(rs, Ok(vec![s.to_owned()]));
    }
}
