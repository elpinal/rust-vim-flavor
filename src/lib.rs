//! A manager of Vim plugins.
#![warn(missing_docs)]
#![feature(ascii_ctype)]

use std::ascii::AsciiExt;
use std::str::Bytes;

/// A parser which parses VimFlavor file.
pub struct Parser<'a> {
    buffer: Bytes<'a>,
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(buffer: &str) -> Parser {
        Parser {
            buffer: buffer.bytes(),
            offset: 0,
        }
    }

    fn skip_whitespaces(&mut self) {
        self.offset = self.buffer
            .position(|ch| !ch.is_ascii_whitespace())
            .map(|n| n + self.offset)
            .unwrap_or(self.buffer.len())
    }

    fn skip_to_next_line(&mut self) {
        let m = self.offset + self.buffer.len();
        self.offset = self.buffer
            .position(|ch| ch == b'\n')
            .map(|n| n + self.offset + 1)
            .unwrap_or(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace() {
        let mut p = Parser::new("  abc");
        p.skip_whitespaces();
        assert_eq!(p.offset, 2);

        p.skip_whitespaces();
        assert_eq!(p.offset, 2);
    }

    #[test]
    fn test_skip_to_next_line() {
        let mut p = Parser::new("aaa\nbbb");
        p.skip_to_next_line();
        assert_eq!(p.offset, 4);

        p.skip_to_next_line();
        assert_eq!(p.offset, 7);

        p.skip_to_next_line();
        assert_eq!(p.offset, 7);
    }
}
