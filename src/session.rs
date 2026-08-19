use std::io::{self, BufRead, Write};

use crate::{eval::eval, parse::Parser};

pub struct Session {}

impl Session {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let mut lines = io::stdin().lock().lines();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            match lines.next() {
                Some(Ok(line)) => self.handle_line(&line),
                Some(Err(e)) => eprintln!("lex error: {e}"),
                None => break,
            }
        }
    }

    fn handle_line(&self, line: &str) {
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
}
