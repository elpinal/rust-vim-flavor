//! A manager of Vim plugins.
#![warn(missing_docs)]
#![allow(unused)]
#![feature(ascii_ctype)]

use std::ascii::AsciiExt;
use std::iter::Enumerate;
use std::str::Bytes;
use std::string::FromUtf8Error;

/// A parser which parses VimFlavor file.
pub struct Parser<'a> {
    buffer: Enumerate<Bytes<'a>>,
    offset: usize,
    byte: Option<u8>,
}

impl<'a> Parser<'a> {
    fn new(buffer: &str) -> Parser {
        let mut p = Parser {
            buffer: buffer.bytes().enumerate(),
            offset: 0,
            byte: None,
        };
        let n = p.buffer.next();
        p.byte = n.map(|t| t.1);
        p
    }

    fn skip_whitespaces(&mut self) {
        if self.byte.map(|b| !b.is_ascii_whitespace()) == Some(true) {
            return;
        }
        if let Some((n, ch)) = self.buffer.find(|&(_, ch)| !ch.is_ascii_whitespace()) {
            self.offset = n;
            self.byte = Some(ch);
        } else {
            self.offset = self.buffer.len();
            self.byte = None;
        }
    }

    fn skip_to_next_line(&mut self) {
        let m = self.offset + self.buffer.len();
        if let Some((n, ch)) = self.buffer.find(|&(_, ch)| ch == b'\n') {
            self.offset = n + 1;
            self.byte = Some(ch);
        } else {
            self.offset = m;
            self.byte = None;
        }
    }

    fn next(&mut self) {
        let n = self.buffer.next();
        self.offset += 1;
        self.byte = n.map(|t| t.1);
    }

    fn next_token(&mut self) -> Option<Token> {
        self.byte.and_then(|b| match b {
            b'#' => {
                self.next();
                Some(Token::Hash)
            }
            b',' => {
                self.next();
                Some(Token::Comma)
            }
            b' ' => {
                self.next();
                self.next_token()
            }
            b if b.is_ascii_alphabetic() => self.read_ident().ok(),
            b'\'' => {
                self.next();
                self.read_string()
            }
            _ => {
                self.next();
                Some(Token::Illegal)
            }
        })
    }

    fn read_ident(&mut self) -> Result<Token, FromUtf8Error> {
        let mut vec = Vec::new();
        while let Some(b) = self.byte {
            if b.is_ascii_alphabetic() {
                vec.push(b);
                self.next();
            } else {
                break;
            }
        }
        String::from_utf8(vec).map(Token::Ident)
    }

    fn read_string(&mut self) -> Option<Token> {
        let mut vec = Vec::new();
        while let Some(b) = self.byte {
            if b != b'\'' {
                vec.push(b);
                self.next();
            } else {
                break;
            }
        }
        if self.byte != Some(b'\'') {
            return None;
        }
        self.next();
        String::from_utf8(vec).map(Token::Str).ok()
    }

    fn parse(&mut self) -> Vec<Token> {
        let mut vec = Vec::new();
        while let Some(t) = self.next_token() {
            vec.push(t);
        }
        vec
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Illegal,
    Hash,
    Ident(String),
    Str(String),
    Comma,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_whitespace() {
        let mut p = Parser::new("  abc");
        p.skip_whitespaces();
        assert_eq!(p.offset, 2);
        assert_eq!(p.byte, Some(b'a'));

        p.skip_whitespaces();
        assert_eq!(p.byte, Some(b'a'));
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
        assert_eq!(p.byte, Some(b' '));
        assert_eq!(p.offset, 0);

        p.next();
        assert_eq!(p.byte, Some(b'a'));
        assert_eq!(p.offset, 1);

        p.next();
        assert_eq!(p.byte, Some(b'b'));
        assert_eq!(p.offset, 2);

        p.next();
        assert_eq!(p.byte, None);
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

        let mut p = Parser::new("#'aaa',");
        assert_eq!(p.next_token(), Some(Token::Hash));
        assert_eq!(p.offset, 1);

        assert_eq!(p.next_token(), Some(Token::Str(String::from("aaa"))));
        assert_eq!(p.offset, 6);

        assert_eq!(p.next_token(), Some(Token::Comma));
        assert_eq!(p.offset, 7);
    }

    #[test]
    fn test_parse() {
        let mut p = Parser::new("## @ #abc#");
        use Token::*;
        assert_eq!(
            p.parse(),
            vec![Hash, Hash, Illegal, Hash, Ident("abc".to_owned()), Hash]
        );
        assert_eq!(p.offset, 10);
    }
}
