use std::io::{self, Write};

fn main() {
    let mut out = io::stdout();

    loop {
        print!("rust-shell> ");
        out.flush().unwrap();
        let mut text = String::new();
        let bytes = io::stdin().read_line(&mut text).unwrap();
        if bytes == 0 {
            break;
        }

        let line = text.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        if line.is_empty() {
            continue;
        }
        if line == "exit" {
            break;
        }
        println!("Command: {:?}\nArgs: {:?}", parts[0], &parts[1..]);
    }
}
