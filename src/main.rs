use std::io::{self, BufRead, Write};

mod lex;
mod parse;

use lex::lex;

use crate::parse::Parser;

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
        let expr = parser.next_expr();
        println!("{:?}", expr)
    }
}
