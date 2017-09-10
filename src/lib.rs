//! A manager of Vim plugins.
#![warn(missing_docs)]
#![feature(ascii_ctype)]

use std::ascii::AsciiExt;
use std::str::Bytes;
use std::string::FromUtf8Error;

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
        self.next().and_then(|b| match b {
            b'#' => Some(Token::Hash),
            b' ' => self.next_token(),
            b if b.is_ascii_alphabetic() => self.read_ident(b).ok(),
            _ => Some(Token::Illegal),
        })
    }

    fn read_ident(&mut self, b: u8) -> Result<Token, FromUtf8Error> {
        let mut vec = Vec::new();
        let mut c = Some(b);
        while let Some(b) = c {
            if b.is_ascii_alphabetic() {
                vec.push(b);
                c = self.next();
            } else {
                break;
            }
        }
        String::from_utf8(vec).map(Token::Ident)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Illegal,
    Hash,
    Ident(String),
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
        let mut p = Parser::new("## @ #");
        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 1);

        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 2);

        assert_eq!(p.next_token(), Some(Token::Illegal));
        assert_eq!(p.offset, 4);

        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 6);

        assert_eq!(p.next_token(), None);
        assert_eq!(p.offset, 6);

        let mut p = Parser::new("abc#de f");
        assert_eq!(p.next_token(), Some(Token::Ident(String::from("abc"))));
        assert_eq!(p.offset, 3);

        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 4);

        assert_eq!(p.next_token(), Some(Token::Ident(String::from("de"))));
        assert_eq!(p.offset, 6);

        assert_eq!(p.next_token(), Some(Token::Ident(String::from("f"))));
        assert_eq!(p.offset, 8);

        assert_eq!(p.next_token(), None);
        assert_eq!(p.offset, 8);
    }
}
