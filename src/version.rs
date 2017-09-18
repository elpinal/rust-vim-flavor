#![allow(unused)]

use std::num;

#[derive(Debug, PartialEq)]
struct Version {
    l: usize,
    m: usize,
    n: usize,
}

impl Version {
    fn new(l: usize, m: usize, n: usize) -> Version {
        Version { l, m, n }
    }

    fn from_str(s: &str) -> Result<Version, FromStrError> {
        let v: Vec<&str> = s.split('.').collect();
        if v.len() != 3 {
            return Err(FromStrError::Split3);
        }
        Ok(Version::new(v[0].parse()?, v[1].parse()?, v[2].parse()?))
    }
}

enum FromStrError {
    Split3,
    Parse(num::ParseIntError),
}

impl From<num::ParseIntError> for FromStrError {
    fn from(e: num::ParseIntError) -> FromStrError {
        FromStrError::Parse(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Version::new(1, 2, 3), Version { l: 1, m: 2, n: 3 });
    }
}
