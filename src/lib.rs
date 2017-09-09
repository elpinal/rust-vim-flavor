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
            .map(|n| n + self.offset)
            .unwrap_or(self.buffer.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut p = Parser::new("  abc");
        p.skip_whitespaces();
        assert_eq!(p.offset, 2);

        p.skip_whitespaces();
        assert_eq!(p.offset, 2);
    }
}
