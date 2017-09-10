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

    fn next(&mut self) -> Option<u8> {
        let n = self.buffer.next();
        if n.is_some() {
            self.offset += 1;
        }
        n
    }

    fn next_token(&mut self) -> Option<Token> {
        self.next().map(|b| match b {
            b'#' => Token::Hash,
            _ => Token::Illegal,
        })
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Illegal,
    Hash,
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

    #[test]
    fn test_next() {
        let mut p = Parser::new(" ab");
        assert_eq!(p.next(), Some(b' '));
        assert_eq!(p.offset, 1);

        assert_eq!(p.next(), Some(b'a'));
        assert_eq!(p.offset, 2);

        assert_eq!(p.next(), Some(b'b'));
        assert_eq!(p.offset, 3);

        assert_eq!(p.next(), None);
        assert_eq!(p.offset, 3);
    }

    #[test]
    fn test_next_token() {
        let mut p = Parser::new("##@");
        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 1);

        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 2);

        assert_eq!(p.next_token(), Some(Token::Illegal));
        assert_eq!(p.offset, 3);

        assert_eq!(p.next_token(), None);
        assert_eq!(p.offset, 3);
    }
}
