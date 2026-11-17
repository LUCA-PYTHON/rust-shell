use std::io::{self, Write};

enum Command {
    Exit,
    Empty,
    Other,
}

fn identify(parts: &[&str]) -> Command {
    if parts.is_empty() {
        return Command::Empty;
    }

    if parts[0] == "exit" {
        return Command::Exit;
    }

    Command::Other
}

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
        match identify(&parts) {
            Command::Exit => break,
            Command::Empty => continue,
            Command::Other => println!("Command: {:?}\nArgs: {:?}", parts[0], &parts[1..]),
        }
    }
}
