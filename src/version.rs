#![allow(unused)]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Version::new(1, 2, 3), Version { l: 1, m: 2, n: 3 });
    }
}
