use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum LexErrorType {
    UnexpectedByte,
    EmptyLiteral,
    Loop,
    Unsupported,
}

#[derive(Debug, PartialEq)]
pub struct LexError {
    pub typ: LexErrorType,
    pub span: Range<usize>,
}

impl LexError {
    fn new(typ: LexErrorType, span: Range<usize>) -> Self {
        Self { typ, span }
    }
}

#[derive(Debug, PartialEq)]
pub enum NumForm {
    Normal,
    Hex,
    Binary,
    Scaled, // Base-10 number with fractional part and/or an exponent 1.5e5/1e10
    Rational,
    Complex,
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
    pub span: Range<usize>,
}

impl Token {
    fn new(typ: TokenType, span: Range<usize>) -> Self {
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

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.eat_spaces();

        let start = self.curr;
        let Some(b) = self.peek() else {
            return Ok(Token::new(TokenType::Eof, start..start));
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
                return Err(LexError::new(LexErrorType::UnexpectedByte, start..start + 1));
            }
        };
        if self.curr > start {
            Ok(Token::new(kind, start..self.curr))
        } else {
            Err(LexError::new(LexErrorType::Loop, start..start + 1))
        }
    }

    fn eat_identifier(&mut self) -> TokenType {
        self.eat_while(|c| c.is_ascii_alphanumeric() || c == b'_');
        TokenType::Identifier
    }

    fn eat_number(&mut self) -> Result<TokenType, LexError> {
        let form = self.eat_number_form()?;
        if self.peek() == Some(b'j') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
            return Err(LexError::new(LexErrorType::Unsupported, self.curr..self.curr + 1));
        }
        Ok(TokenType::Num(form))
    }

    fn eat_number_form(&mut self) -> Result<NumForm, LexError> {
        if self.peek().unwrap() == b'0' {
            match self.peek_ahead(1) {
                Some(b'x') => {
                    self.skip(2);
                    self.eat_expect_while(|c| c.is_ascii_hexdigit(), LexErrorType::EmptyLiteral)?;
                    return Ok(NumForm::Hex);
                }
                Some(b'b') => {
                    self.skip(2);
                    self.eat_expect_while(|c| c == b'0' || c == b'1', LexErrorType::EmptyLiteral)?;
                    return Ok(NumForm::Binary);
                }
                _ => {}
            }
        }
        self.eat_while(|c| c.is_ascii_digit());

        if self.peek() == Some(b'r') && self.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) {
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

    fn eat_expect_while(&mut self, pred: impl Fn(u8) -> bool, e: LexErrorType) -> Result<(), LexError> {
        let before = self.curr;
        self.eat_while(pred);
        if self.curr > before {
            Ok(())
        } else {
            Err(LexError::new(e, self.curr..self.curr + 1))
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

pub fn lex(src: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();
    loop {
        let t = lexer.next_token()?;
        let done = t.typ == TokenType::Eof;
        tokens.push(t);
        if done {
            return Ok(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::{TokenType::Num, *};

    fn lex_no_eof(src: &str) -> Vec<Token> {
        let mut result = lex(src).unwrap();
        result.pop();
        result
    }

    #[test]
    pub fn lex_base10_number() {
        assert_eq!(
            lex_no_eof("123"),
            vec![Token::new(TokenType::Num(NumForm::Normal), 0..3)]
        )
    }

    #[test]
    pub fn lex_hex_number() {
        assert_eq!(
            lex_no_eof("0x123ABCdef"),
            vec![Token::new(TokenType::Num(NumForm::Hex), 0..11)]
        )
    }

    #[test]
    pub fn lex_binary_number() {
        assert_eq!(
            lex_no_eof("0b01101"),
            vec![Token::new(TokenType::Num(NumForm::Binary), 0..7)]
        )
    }

    #[test]
    pub fn lex_rational_number() {
        assert_eq!(
            lex_no_eof("34r10"),
            vec![Token::new(TokenType::Num(NumForm::Rational), 0..5)]
        )
    }

    #[test]
    pub fn lex_decimal_number() {
        assert_eq!(
            lex_no_eof("1.234"),
            vec![Token::new(TokenType::Num(NumForm::Scaled), 0..5)]
        )
    }

    #[test]
    pub fn lex_exponent_number() {
        assert_eq!(
            lex_no_eof("5e10"),
            vec![Token::new(TokenType::Num(NumForm::Scaled), 0..4)]
        )
    }

    #[test]
    pub fn lex_decimal_exponent_number() {
        assert_eq!(
            lex_no_eof("1.4e5"),
            vec![Token::new(TokenType::Num(NumForm::Scaled), 0..5)]
        )
    }

    #[test]
    pub fn lex_complex_returns_err() {
        assert!(lex("1j2").is_err());
        assert!(lex("1.5j2.0").is_err());
        assert!(lex("1r3j2r5").is_err());
        assert!(lex("1e5j2e10").is_err());
    }
}
