#![allow(unused)]

use std::num;
use std::str::FromStr;

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
}

impl FromStr for Version {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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

    #[test]
    fn test_from_str() {
        assert_eq!(
            Version::from_str("3.21.0").ok(),
            Some(Version { l: 3, m: 21, n: 0 })
        );

        // str's parse method verison.
        assert_eq!("3.21.0".parse().ok(), Some(Version { l: 3, m: 21, n: 0 }));
    }
}
