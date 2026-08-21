use std::io::{self, BufRead, Write};

use crate::{
    error::{Error, ErrorKind},
    eval::eval,
    parse::Parser,
};

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
                Some(Ok(line)) => match self.handle_line(&line) {
                    Ok(_) => {}
                    Err(err) => {
                        if err.span.from == err.span.to {
                            eprintln!("{}^ error: {}", " ".repeat(err.span.from + 2), err.kind)
                        } else if err.span.from == err.span.to - 1 {
                            eprintln!("{}^ error: {}", " ".repeat(err.span.from + 2), err.kind)
                        } else {
                            let spaces = " ".repeat(err.span.from + 2);
                            let carets = "~".repeat(err.span.to - (err.span.from + 1));
                            eprintln!("{spaces}^{carets}^ error: {}", err.kind);
                        }
                    }
                },
                Some(Err(e)) => eprintln!("I/O error: {}", e),
                None => break,
            }
        }
    }

    fn handle_line(&self, line: &str) -> Result<(), Error> {
        if let Some(command) = line.trim_start().strip_prefix(COMMAND_PREFIX) {
            self.handle_command(command)?;
            return Ok(());
        }

        let mut parser = Parser::new(line)?;
        loop {
            match parser.next_expr()? {
                Some(expr) => match eval(&expr) {
                    Ok(val) => println!("{val}"),
                    Err(e) => eprintln!("error: {e}"),
                },
                None => break,
            }
        }
        Ok(())
    }

    fn handle_command(&self, line: &str) -> Result<String, Error> {
        match line {
            "ping" => Ok("pong".to_string()),
            _ => Err(Error::new(ErrorKind::InvalidCommand, 0, line.len())),
        }
    }
}
