#![allow(unused)]

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

#[derive(Debug)]
enum FromStrError {
    Split3,
    Parse(ParseIntError),
}

impl fmt::Display for FromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FromStrError::Split3 => {
                write!(f, "string does not consist of three numbers split by dots")
            }
            FromStrError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl Error for FromStrError {
    fn description(&self) -> &str {
        match *self {
            FromStrError::Split3 => "string does not consist of three numbers split by dots",
            FromStrError::Parse(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            FromStrError::Split3 => None,
            FromStrError::Parse(ref e) => e.cause(),
        }
    }
}

impl From<ParseIntError> for FromStrError {
    fn from(e: ParseIntError) -> FromStrError {
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
            Some(Version::new(3, 21, 0))
        );

        // str's parse method verison.
        assert_eq!("3.21.0".parse().ok(), Some(Version::new(3, 21, 0)));

        assert!("".parse::<Version>().is_err());
        assert!("3210".parse::<Version>().is_err());
        assert!("3.210".parse::<Version>().is_err());
        assert!("3.2.1.0".parse::<Version>().is_err());

        assert!("3.-2.1".parse::<Version>().is_err());
        assert!("3.-.1".parse::<Version>().is_err());
        assert!("3.a.1".parse::<Version>().is_err());
    }

    #[test]
    fn test_sort() {
        let vec = vec![Version::new(1, 2, 3), Version::new(1, 12, 2)];
        let mut s = vec.clone();
        s.sort();
        assert_eq!(s, vec);
    }
}
