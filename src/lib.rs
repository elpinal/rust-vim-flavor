#![warn(missing_docs)]

pub struct Parser {
    buffer: String,
    offset: usize,
}

impl Parser {
    fn new(buffer: &str) -> Parser {
        Parser {
            buffer: String::from(buffer),
            offset: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
