use std::io::{self, Write};

fn main() {
    let mut out = io::stdout();

    print!("rust-shell> ");
    out.flush().unwrap();

    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    let clean = text.trim();
    println!("You typed: '{}'", clean);
}