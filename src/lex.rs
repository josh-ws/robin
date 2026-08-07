use std::{iter::Peekable, path::Display, str::Chars};

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Token {
    Num(String),
    Identifier(String),
    Op(Op),
    LParen,
    RParen,
    Eof,
}

pub fn lex(src: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = src.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            c if c.is_whitespace() => {
                chars.next();
            }
            c if c.is_ascii_digit() => tokens.push(lex_number(&mut chars)),
            c if c.is_ascii_alphabetic() || c == '_' => tokens.push(lex_identifier(&mut chars)),
            '(' => tokens.push(consume(&mut chars, Token::LParen)),
            ')' => tokens.push(consume(&mut chars, Token::RParen)),
            '+' => tokens.push(consume(&mut chars, Token::Op(Op::Add))),
            '-' => tokens.push(consume(&mut chars, Token::Op(Op::Sub))),
            '*' => tokens.push(consume(&mut chars, Token::Op(Op::Mul))),
            '/' => tokens.push(consume(&mut chars, Token::Op(Op::Div))),
            _ => panic!("unexpected character {c}"),
        };
    }
    tokens.push(Token::Eof);
    tokens
}

fn consume(src: &mut Peekable<Chars>, result: Token) -> Token {
    src.next();
    result
}

fn lex_identifier(src: &mut Peekable<Chars>) -> Token {
    let mut s = String::new();
    while let Some(&c) = src.peek() {
        if c.is_ascii_alphanumeric() || c == '_' {
            s.push(c);
            src.next();
        } else {
            break;
        }
    }
    Token::Identifier(s)
}

fn lex_number(src: &mut Peekable<Chars>) -> Token {
    let mut s = String::new();
    while let Some(&c) = src.peek() {
        if c.is_ascii_digit() {
            s.push(c);
            src.next();
        } else {
            break;
        }
    }
    Token::Num(s)
}
