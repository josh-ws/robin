use std::io::{self, BufRead, Write};

fn main() {
    let mut lines = io::stdin().lock().lines();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        match lines.next() {
            Some(Ok(line)) => handle_line(&line),
            Some(Err(e)) => panic!("input error"),
            None => break,
        }
    }
}

fn handle_line(line: &str) {
    println!("{}", line)
}
