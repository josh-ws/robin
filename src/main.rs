mod error;
mod eval;
mod lex;
mod numeric;
mod parse;
mod session;

use session::Session;

fn main() {
    let session = Session::new();
    session.run();
}
