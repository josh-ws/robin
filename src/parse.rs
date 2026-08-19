use std::{
    fmt::{self, Binary},
    ops::Range,
};

use dashu::integer::IBig;

use crate::{
    lex::{LexError, Lexer, NumForm, Token, TokenType},
    numeric::Numeric,
};

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Pow,
}

#[derive(Debug)]
pub enum ExprType {
    Number(Numeric),
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
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
        if let Some(op) = self.lookup_unary(&self.token) {
            let tok = self.bump()?;
            let operand = self.parse_expr()?;
            let span = tok.span.start..operand.span.end;
            return Ok(Expr::new(
                ExprType::Unary {
                    op,
                    operand: Box::new(operand),
                },
                span,
            ));
        }
        let lhs = self.parse_operand()?;
        let Some(def) = self.lookup_binary(&self.token) else {
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
            TokenType::Num(form) => {
                let (radix, digits) = match form {
                    NumForm::Normal => (10, slice),
                    NumForm::Hex => (16, &slice[2..]),
                    NumForm::Binary => (2, &slice[2..]),
                    NumForm::Scaled | NumForm::Rational | NumForm::Complex => unimplemented!(),
                };
                let val = match IBig::from_str_radix(digits, radix) {
                    Ok(n) => Numeric::from_big(n),
                    Err(_) => unimplemented!(),
                };
                ExprType::Number(val)
            }
            _ => unimplemented!(),
        };
        Ok(Expr::new(typ, tok.span))
    }

    fn lookup_unary(&self, tok: &Token) -> Option<UnaryOp> {
        match tok.typ {
            TokenType::Minus => Some(UnaryOp::Neg),
            _ => None,
        }
    }

    fn lookup_binary(&self, tok: &Token) -> Option<BinaryOp> {
        match tok.typ {
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Sub),
            TokenType::Star => Some(BinaryOp::Mul),
            TokenType::Identifier => match &self.src[tok.span.start..tok.span.end] {
                "add" => Some(BinaryOp::Add),
                "sub" => Some(BinaryOp::Sub),
                "mul" => Some(BinaryOp::Mul),
                "pow" => Some(BinaryOp::Pow),
                _ => None,
            },
            _ => None,
        }
    }

    fn bump(&mut self) -> Result<Token, LexError> {
        let next = self.lexer.next_token()?;
        Ok(std::mem::replace(&mut self.token, next))
    }
}
