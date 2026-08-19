use std::io::{self, BufRead, Write};

use crate::{eval::eval, parse::Parser};

const COMMAND_PREFIX: char = ')';

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
        if let Some(command) = line.trim_start().strip_prefix(COMMAND_PREFIX) {
            match self.handle_command(command) {
                Ok(str) => println!("{str}"),
                Err(err) => eprintln!("error: {err}"),
            }
            return;
        }

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

    fn handle_command(&self, line: &str) -> Result<String, String> {
        match line {
            "ping" => Ok("pong".to_string()),
            _ => Err(format!("unrecognized command `{}`", line)),
        }
    }
}
