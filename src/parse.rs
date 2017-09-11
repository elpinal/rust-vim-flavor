use std::ascii::AsciiExt;
use std::iter::Enumerate;
use std::str::Bytes;
use std::str::Utf8Error;
use std::string::FromUtf8Error;

/// A parser which parses VimFlavor file.
pub struct Parser<'a> {
    buffer: Enumerate<Bytes<'a>>,
    offset: usize,
    byte: Option<u8>,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &str) -> Parser {
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
        let (n, ch) = self.buffer
            .find(|&(_, ch)| ch == b'\n')
            .map(|(n, ch)| (n + 1, Some(ch)))
            .unwrap_or((m, None));
        self.offset = n;
        self.byte = ch;
    }

    fn next(&mut self) {
        let n = self.buffer.next();
        self.offset += 1;
        self.byte = n.map(|t| t.1);
    }

    fn next_token(&mut self) -> Result<Token, ParseError> {
        self.byte.ok_or(ParseError::EOF).and_then(|b| {
            if b.is_ascii_alphabetic() {
                return self.read_ident();
            }
            self.next();
            match b {
                b'#' => Ok(Token::Hash),
                b',' => Ok(Token::Comma),
                b' ' => self.next_token(),
                b'\'' => self.read_string(),
                _ => Ok(Token::Illegal),
            }
        })
    }

    fn read_ident(&mut self) -> Result<Token, ParseError> {
        let mut vec = Vec::new();
        while let Some(b) = self.byte {
            if b.is_ascii_alphabetic() {
                vec.push(b);
                self.next();
            } else {
                break;
            }
        }
        let s = String::from_utf8(vec)?;
        if s == "flavor" {
            return Ok(Token::Flavor);
        }
        Ok(Token::Ident(s))
    }

    fn read_string(&mut self) -> Result<Token, ParseError> {
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
            return Err(ParseError::Terminate);
        }
        self.next();
        Ok(String::from_utf8(vec).map(Token::Str)?)
    }

    pub fn parse(&mut self) -> Result<Vec<Flavor>, ParseError> {
        let mut vec = Vec::new();
        while let Some(t) = self.next_token().ok() {
            match t {
                Token::Hash => self.skip_to_next_line(),
                Token::Flavor => {
                    let t = self.next_token()?;
                    match t {
                        Token::Str(s) => vec.push(Flavor { repo: s }),
                        _ => {
                            return Err(ParseError::TypeMismatch);
                        }
                    }
                }
                _ => (),
            }
        }
        Ok(vec)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Illegal,
    Hash,
    Ident(String),
    Str(String),
    Comma,
    Flavor,
}

#[derive(Debug, PartialEq)]
pub struct Flavor {
    repo: String,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Utf8(Utf8Error),
    Terminate,
    EOF,
    TypeMismatch,
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> ParseError {
        ParseError::Utf8(err.utf8_error())
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
        assert_eq!(p.next_token(), Ok(Token::Hash));
        assert_eq!(p.offset, 1);

        assert_eq!(p.next_token(), Ok(Token::Hash));
        assert_eq!(p.offset, 2);

        assert_eq!(p.next_token(), Ok(Token::Illegal));
        assert_eq!(p.offset, 4);

        assert_eq!(p.next_token(), Ok(Token::Hash));
        assert_eq!(p.offset, 6);

        assert_eq!(p.next_token().err(), Some(ParseError::EOF));
        assert_eq!(p.offset, 6);

        let mut p = Parser::new("abc#de f");
        assert_eq!(p.next_token(), Ok(Token::Ident(String::from("abc"))));
        assert_eq!(p.offset, 3);

        assert_eq!(p.next_token(), Ok(Token::Hash));
        assert_eq!(p.offset, 4);

        assert_eq!(p.next_token(), Ok(Token::Ident(String::from("de"))));
        assert_eq!(p.offset, 6);

        assert_eq!(p.next_token(), Ok(Token::Ident(String::from("f"))));
        assert_eq!(p.offset, 8);

        assert_eq!(p.next_token().err(), Some(ParseError::EOF));
        assert_eq!(p.offset, 8);

        let mut p = Parser::new("#'aaa',");
        assert_eq!(p.next_token(), Ok(Token::Hash));
        assert_eq!(p.offset, 1);

        assert_eq!(p.next_token(), Ok(Token::Str(String::from("aaa"))));
        assert_eq!(p.offset, 6);

        assert_eq!(p.next_token(), Ok(Token::Comma));
        assert_eq!(p.offset, 7);
    }

    #[test]
    fn test_parse() {
        let s = "# comment of flavor file\n\
                 flavor 'repo'";
        let mut p = Parser::new(s);
        assert_eq!(p.parse(), Ok(vec![Flavor { repo: "repo".to_owned() }]));
        assert_eq!(p.offset, s.len() + 1);

        let s = "flavor flavor";
        let mut p = Parser::new(s);
        assert!(p.parse().is_err());
    }
}
