use std::io::{self, Write};

fn main() {
    let mut out = io::stdout();

    print!("rust-shell> ");
    out.flush().unwrap();
}
