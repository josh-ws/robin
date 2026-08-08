use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Num,
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

    pub fn next_token(&mut self) -> Token {
        // TODO(jw) fix panics
        self.eat_spaces();

        let start = self.curr;
        let Some(b) = self.peek() else {
            return Token::new(TokenType::Eof, start..start);
        };
        let kind = match b {
            b if b.is_ascii_alphabetic() || b == b'_' => self.eat_identifier(),
            b if b.is_ascii_digit() => self.eat_number(),
            b'*' => self.consume(TokenType::Star),
            b'/' => self.consume(TokenType::Slash),
            b'-' => self.consume(TokenType::Minus),
            b'+' => self.consume(TokenType::Plus),
            b'(' => self.consume(TokenType::LParen),
            b')' => self.consume(TokenType::RParen),
            b'\n' => self.consume(TokenType::Newline),
            _ => panic!("bailing"),
        };
        debug_assert!(self.curr > start);
        Token::new(kind, start..self.curr)
    }

    fn eat_identifier(&mut self) -> TokenType {
        self.eat_while(|c| c.is_ascii_alphanumeric() || c == b'_');
        TokenType::Identifier
    }

    fn eat_number(&mut self) -> TokenType {
        self.eat_while(|c| c.is_ascii_digit());
        TokenType::Num
    }

    fn eat_spaces(&mut self) {
        self.eat_while(|c| c != b'\n' && c.is_ascii_whitespace());
    }

    fn eat_while(&mut self, pred: impl Fn(u8) -> bool) {
        while let Some(b) = self.peek() {
            if !pred(b) {
                break;
            }
            self.bump();
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.curr).copied()
    }

    fn bump(&mut self) {
        self.curr += 1;
    }

    fn consume(&mut self, typ: TokenType) -> TokenType {
        self.bump();
        typ
    }
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(src);
    let mut tokens = Vec::new();
    loop {
        let t = lexer.next_token();
        let done = t.typ == TokenType::Eof;
        tokens.push(t);
        if done {
            return tokens;
        }
    }
}
