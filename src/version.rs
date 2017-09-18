#![allow(unused)]

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
