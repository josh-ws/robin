use std::{fmt, ops::Range};

use crate::lex::{LexError, Lexer, NumForm, Token, TokenType};

#[derive(Debug)]
pub enum OpType {
    Plus,
}

#[derive(Debug)]
pub enum OpParity {
    Unary,
    Binary,
}

#[derive(Debug)]
pub struct Op {
    pub typ: OpType,
    pub parity: OpParity,
}

impl Op {
    fn new(typ: OpType, parity: OpParity) -> Self {
        Self { typ, parity }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Numeric {
    Int(i64),
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Numeric::Int(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Debug)]
pub enum ExprType {
    Number(Numeric),
    Binary { op: Op, lhs: Box<Expr>, rhs: Box<Expr> },
}

#[derive(Debug)]
pub struct Expr {
    pub typ: ExprType,
    pub span: Range<usize>,
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
        let expr = self.parse_expr()?;
        match self.token.typ {
            TokenType::Newline | TokenType::Eof => Ok(Some(expr)),
            _ => unimplemented!(), // TODO(jw) parse error
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, LexError> {
        let lhs = self.parse_operand()?;
        let Some(def) = self.lookup_op(&self.token) else {
            return Ok(lhs);
        };
        self.bump()?;
        let rhs = self.parse_expr()?;
        let span = lhs.span.start..rhs.span.end;
        Ok(Expr::new(
            ExprType::Binary {
                op: def,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            span,
        ))
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

    fn lookup_op(&self, tok: &Token) -> Option<Op> {
        match tok.typ {
            TokenType::Plus => Some(Op::new(OpType::Plus, OpParity::Binary)),
            _ => None,
        }
    }

    fn bump(&mut self) -> Result<Token, LexError> {
        let next = self.lexer.next_token()?;
        Ok(std::mem::replace(&mut self.token, next))
    }
}
