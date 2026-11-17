use std::io::{self, Write};

fn parse_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}

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
        let parts = parse_line(line);
        if parts.is_empty() {
            continue;
        }
        if parts[0] == "exit" {
            break;
        }
        println!("Command: {:?}\nArgs: {:?}", parts[0], &parts[1..]);
    }
}
