use std::{fmt::Display, ops::Range};

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken,
    EmptyLiteral,
    Unsupported,
    ExpectedOperand,
    InvalidNumberFormat,
    InvalidCommand,
    InfiniteLoop,
    DivisionByZero,
    UnexpectedEof,
    TrailingInput,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            ErrorKind::UnexpectedToken => "unexpected token",
            ErrorKind::EmptyLiteral => "empty literal",
            ErrorKind::Unsupported => "unsupported operation",
            ErrorKind::InvalidNumberFormat => "number in invalid format",
            ErrorKind::InvalidCommand => "unrecognized command",
            ErrorKind::InfiniteLoop => "infinite loop detected! please raise a bug",
            ErrorKind::DivisionByZero => "division by zero",
            ErrorKind::ExpectedOperand => "expected operand",
            ErrorKind::UnexpectedEof => "unexpected eof",
            ErrorKind::TrailingInput => "trailing input",
        };
        write!(f, "{}", err)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

impl Span {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn merge(lhs: Self, rhs: Self) -> Self {
        Self::new(lhs.from.min(rhs.from), lhs.to.max(rhs.to))
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

impl Error {
    pub fn new(kind: ErrorKind, from: usize, to: usize) -> Self {
        Self {
            kind,
            span: Span::new(from, to),
        }
    }
}
