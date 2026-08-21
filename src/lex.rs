use std::ops::Range;

use crate::error::{Error, ErrorKind, Span};

#[derive(Debug, PartialEq)]
pub enum NumForm {
    Normal,
    Hex,
    Binary,
    Scaled, // Base-10 number with fractional part and/or an exponent 1.5e5/1e10
    Rational,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num(NumForm),
    Identifier,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Newline,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub typ: TokenType,
    pub span: Span,
}

impl Token {
    fn new(typ: TokenType, span: Span) -> Self {
        Self { typ, span }
    }
}

pub struct Lexer<'a> {
    src: &'a [u8],
    curr: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.as_bytes(),
            curr: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, Error> {
        self.eat_spaces();

        let start = self.curr;
        let Some(b) = self.peek() else {
            return Ok(Token::new(TokenType::Eof, Span::new(start, start)));
        };
        let kind = match b {
            b if b.is_ascii_alphabetic() || b == b'_' => self.eat_identifier(),
            b if b.is_ascii_digit() => self.eat_number()?,
            b'*' => self.consume(TokenType::Star),
            b'/' => self.consume(TokenType::Slash),
            b'-' => self.consume(TokenType::Minus),
            b'+' => self.consume(TokenType::Plus),
            b'(' => self.consume(TokenType::LParen),
            b')' => self.consume(TokenType::RParen),
            b'\n' => self.consume(TokenType::Newline),
            _ => {
                return Err(Error::new(ErrorKind::UnexpectedToken, start, start + 1));
            }
        };
        if self.curr > start {
            Ok(Token::new(kind, Span::new(start, self.curr)))
        } else {
            Err(Error::new(ErrorKind::InfiniteLoop, start, start + 1))
        }
    }

    fn eat_identifier(&mut self) -> TokenType {
        self.eat_while(|c| c.is_ascii_alphanumeric() || c == b'_');
        TokenType::Identifier
    }

    fn eat_number(&mut self) -> Result<TokenType, Error> {
        let form = self.eat_number_form()?;
        if self.peek() == Some(b'j') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
            return Err(Error::new(ErrorKind::Unsupported, self.curr, self.curr + 1));
        }
        Ok(TokenType::Num(form))
    }

    fn eat_number_form(&mut self) -> Result<NumForm, Error> {
        if self.peek().unwrap() == b'0' {
            match self.peek_ahead(1) {
                Some(b'x') => {
                    self.skip(2);
                    self.eat_expect_while(|c| c.is_ascii_hexdigit(), ErrorKind::EmptyLiteral)?;
                    return Ok(NumForm::Hex);
                }
                Some(b'b') => {
                    self.skip(2);
                    self.eat_expect_while(|c| c == b'0' || c == b'1', ErrorKind::EmptyLiteral)?;
                    return Ok(NumForm::Binary);
                }
                _ => {}
            }
        }
        self.eat_while(|c| c.is_ascii_digit());

        if self.peek() == Some(b'/') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
            self.skip(1);
            self.eat_while(|c| c.is_ascii_digit());
            return Ok(NumForm::Rational);
        }

        let mut form = NumForm::Normal;
        if self.peek() == Some(b'.') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
            self.skip(1);
            self.eat_while(|c| c.is_ascii_digit());
            form = NumForm::Scaled;
        }
        if self.peek() == Some(b'j') {
            return Err(Error::new(ErrorKind::Unsupported, self.curr, self.curr + 1));
        }
        if self.peek() == Some(b'e') {
            let digits = match self.peek_ahead(1) {
                Some(b'+' | b'-') => 2,
                _ => 1,
            };
            if self.peek_ahead(digits).is_some_and(|c| c.is_ascii_digit()) {
                self.skip(digits);
                self.eat_while(|c| c.is_ascii_digit());
                form = NumForm::Scaled;
            }
        }

        Ok(form)
    }

    fn eat_spaces(&mut self) {
        self.eat_while(|c| c != b'\n' && c.is_ascii_whitespace());
    }

    fn eat_while(&mut self, pred: impl Fn(u8) -> bool) {
        while let Some(b) = self.peek() {
            if !pred(b) {
                break;
            }
            self.skip(1);
        }
    }

    fn eat_expect_while(&mut self, pred: impl Fn(u8) -> bool, e: ErrorKind) -> Result<(), Error> {
        let before = self.curr;
        self.eat_while(pred);
        if self.curr > before {
            Ok(())
        } else {
            Err(Error::new(e, self.curr, self.curr + 1))
        }
    }

    fn peek_ahead(&self, by: usize) -> Option<u8> {
        self.src.get(self.curr + by).copied()
    }

    fn peek(&self) -> Option<u8> {
        self.peek_ahead(0)
    }

    fn skip(&mut self, by: usize) {
        self.curr += by;
    }

    fn consume(&mut self, typ: TokenType) -> TokenType {
        self.skip(1);
        typ
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::{TokenType::Num, *};

    fn lex_first(src: &str) -> Result<Token, Error> {
        let mut lexer = Lexer::new(src);
        lexer.next_token()
    }

    #[test]
    pub fn lex_numbers() {
        let cases = [
            ("1234567890", NumForm::Normal, Span::new(0, 10)),
            ("0x123ABCdef", NumForm::Hex, Span::new(0, 11)),
            ("0b01101", NumForm::Binary, Span::new(0, 7)),
            ("34/10", NumForm::Rational, Span::new(0, 5)),
            ("1.234", NumForm::Scaled, Span::new(0, 5)),
            ("5e10", NumForm::Scaled, Span::new(0, 4)),
            ("1.4e5", NumForm::Scaled, Span::new(0, 5)),
        ];
        for (src, form, span) in cases {
            assert_eq!(lex_first(src).unwrap(), Token::new(Num(form), span), "lexing {src:?}");
        }
    }

    #[test]
    pub fn lex_complex_returns_err() {
        let cases = ["1j2", "1.5j2.0", "1/3j2/5", "1e5j2e10"];
        for src in cases {
            assert!(lex_first(src).is_err(), "lexing {src:?}");
        }
    }
}
