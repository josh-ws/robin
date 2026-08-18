use std::ops::Range;

use crate::lex::{LexError, Lexer, NumForm, Token, TokenType};

#[derive(Debug)]
pub enum Numeric {
    Int(i64),
}

#[derive(Debug)]
pub enum ExprType {
    Number(Numeric),
}

#[derive(Debug)]
pub struct Expr {
    typ: ExprType,
    span: Range<usize>,
}

impl Expr {
    fn new(typ: ExprType, span: Range<usize>) -> Self {
        Self { typ, span }
    }
}

pub struct Parser<'a> {
    src: &'a str,
    lexer: Lexer<'a>,
    token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let mut lexer = Lexer::new(src);
        let token = lexer.next_token().expect("failed to read a token from lexer"); // TODO(jw) remove panic
        Self { src, lexer, token }
    }

    // TODO(jw) create wrapped error type
    // TODO(jw) fix obvious panics
    pub fn next_expr(&mut self) -> Result<Option<Expr>, LexError> {
        while self.token.typ == TokenType::Newline {
            self.bump()?;
        }
        if self.token.typ == TokenType::Eof {
            return Ok(None);
        }
        let expr = self.parse_operand()?;
        match self.token.typ {
            TokenType::Newline | TokenType::Eof => Ok(Some(expr)),
            _ => unimplemented!(), // TODO(jw) parse error
        }
    }

    fn parse_operand(&mut self) -> Result<Expr, LexError> {
        let tok = self.bump()?;
        let slice = &self.src[tok.span.start..tok.span.end];
        let typ = match tok.typ {
            TokenType::Num(form) => match form {
                NumForm::Normal | NumForm::Hex | NumForm::Binary => {
                    let (radix, digits) = match form {
                        NumForm::Normal => (10, slice),
                        NumForm::Hex => (16, slice.trim_start_matches("0x")),
                        NumForm::Binary => (2, slice.trim_start_matches("0b")),
                        _ => unimplemented!(),
                    };
                    let val = match i64::from_str_radix(digits, radix) {
                        Ok(n) => Numeric::Int(n),
                        Err(_) => todo!("promotion"),
                    };
                    ExprType::Number(val)
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        };
        Ok(Expr::new(typ, tok.span))
    }

    fn bump(&mut self) -> Result<Token, LexError> {
        let next = self.lexer.next_token()?;
        Ok(std::mem::replace(&mut self.token, next))
    }
}
