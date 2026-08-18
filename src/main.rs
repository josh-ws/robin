use std::io::{self, BufRead, Write};

mod eval;
mod lex;
mod parse;

use lex::lex;

use crate::{eval::eval, parse::Parser};

fn main() {
    let mut lines = io::stdin().lock().lines();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match lines.next() {
            Some(Ok(line)) => handle_line(&line),
            Some(Err(e)) => eprintln!("lex error: {e}"),
            None => break,
        }
    }
}

fn handle_line(line: &str) {
    let mut parser = Parser::new(line);
    loop {
        match parser.next_expr() {
            Ok(Some(expr)) => match eval(&expr) {
                Ok(val) => println!("{val}"),
                Err(e) => eprintln!("error: {e}"),
            },
            Ok(None) => break,
            Err(e) => {
                eprintln!("error: {e:?}");
                break;
            }
        }
    }
}
