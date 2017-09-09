//! A manager of Vim plugins.
#![warn(missing_docs)]
#![feature(ascii_ctype)]

/// A parser which parses VimFlavor file.
pub struct Parser {
    buffer: String,
    offset: usize,
}

use std::ascii::AsciiExt;

impl Parser {
    fn new(buffer: &str) -> Parser {
        Parser {
            buffer: String::from(buffer),
            offset: 0,
        }
    }

    fn skip_whitespaces(&mut self) {
        self.offset = self.buffer
            .bytes()
            .skip(self.offset)
            .position(|ch| !ch.is_ascii_whitespace())
            .unwrap_or(self.buffer.len())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
