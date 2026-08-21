use std::str::FromStr;

use dashu::{integer::IBig, rational::RBig};

use crate::{
    error::{Error, ErrorKind, Span},
    lex::{Lexer, NumForm, Token, TokenType},
    numeric::Numeric,
};

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Pow,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Expr {
    pub typ: ExprType,
    pub span: Span,
}

impl Expr {
    fn new(typ: ExprType, span: Span) -> Self {
        Self { typ, span }
    }
}

pub struct Parser<'a> {
    src: &'a str,
    lexer: Lexer<'a>,
    token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Result<Self, Error> {
        let mut lexer = Lexer::new(src);
        let token = lexer.next_token()?;
        Ok(Self { src, lexer, token })
    }

    pub fn next_expr(&mut self) -> Result<Option<Expr>, Error> {
        while self.token.typ == TokenType::Newline {
            self.bump()?;
        }
        if self.token.typ == TokenType::Eof {
            return Ok(None);
        }
        let expr = self.parse_expr()?;
        match self.token.typ {
            TokenType::Newline | TokenType::Eof => Ok(Some(expr)),
            _ => Err(Error::new(
                ErrorKind::TrailingInput,
                self.token.span.from,
                self.token.span.to,
            )),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        if let Some(op) = self.lookup_unary(&self.token) {
            let tok = self.bump()?;
            let operand = self.parse_expr()?;
            let span = Span::merge(tok.span, operand.span);
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
        let span = Span::merge(lhs.span, rhs.span);
        Ok(Expr::new(
            ExprType::Binary {
                op: def,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
            span,
        ))
    }

    fn parse_operand(&mut self) -> Result<Expr, Error> {
        let tok = self.bump()?;
        let span = tok.span;
        let slice = &self.src[span.from..span.to];
        let bad_num = |_| Error::new(ErrorKind::InvalidNumberFormat, span.from, span.to);
        let typ = match tok.typ {
            TokenType::Num(form) => {
                let val = match form {
                    NumForm::Normal => Numeric::from_big(IBig::from_str_radix(slice, 10).map_err(bad_num)?),
                    NumForm::Hex => Numeric::from_big(IBig::from_str_radix(&slice[2..], 16).map_err(bad_num)?),
                    NumForm::Binary => Numeric::from_big(IBig::from_str_radix(&slice[2..], 2).map_err(bad_num)?),
                    NumForm::Scaled => Numeric::from_rat(RBig::from_str_decimal(slice).map_err(bad_num)?),
                    NumForm::Rational => {
                        let (_, den) = slice.split_once('/').expect("parse bug splitting '/', please report");
                        if den.bytes().all(|b| b == b'0') {
                            return Err(Error::new(ErrorKind::DivisionByZero, span.from, span.to));
                        }
                        Numeric::from_rat(RBig::from_str(slice).map_err(bad_num)?)
                    }
                };
                ExprType::Number(val)
            }
            TokenType::Eof => {
                return Err(Error::new(ErrorKind::UnexpectedEof, span.from, span.to));
            }
            _ => {
                return Err(Error::new(ErrorKind::ExpectedOperand, span.from, span.to));
            }
        };
        Ok(Expr::new(typ, tok.span))
    }

    fn lookup_unary(&self, tok: &Token) -> Option<UnaryOp> {
        match tok.typ {
            TokenType::Minus => Some(UnaryOp::Neg),
            TokenType::Identifier => match self.slice(tok.span) {
                "neg" => Some(UnaryOp::Neg),
                _ => None,
            },
            _ => None,
        }
    }

    fn lookup_binary(&self, tok: &Token) -> Option<BinaryOp> {
        match tok.typ {
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Sub),
            TokenType::Star => Some(BinaryOp::Mul),
            TokenType::Identifier => match self.slice(tok.span) {
                "add" => Some(BinaryOp::Add),
                "sub" => Some(BinaryOp::Sub),
                "mul" => Some(BinaryOp::Mul),
                "pow" => Some(BinaryOp::Pow),
                _ => None,
            },
            _ => None,
        }
    }

    fn slice(&self, span: Span) -> &'a str {
        &self.src[span.from..span.to]
    }

    fn bump(&mut self) -> Result<Token, Error> {
        let next = self.lexer.next_token()?;
        Ok(std::mem::replace(&mut self.token, next))
    }
}
