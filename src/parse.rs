use std::ops::Range;

use crate::lex::{LexError, Lexer, NumForm, Token, TokenType, lex};

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
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            lexer: Lexer::new(src),
        }
    }

    // TODO(jw) create wrapped error type
    // TODO(jw) fix obvious panics
    pub fn next_expr(&mut self) -> Result<Expr, LexError> {
        let next = self.lexer.next_token()?;
        let slice = &self.src[next.span.clone()];
        let typ = match next.typ {
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
        Ok(Expr::new(typ, next.span.clone()))
    }
}
