use std::io::{self, Write};

// Enumeration, Datentyp mit mehreren Varianten eines Werts
enum Command<'a> {
    Exit,
    Empty,
    Other(&'a str, &'a [&'a str]),
}

// Rausfiltern welcher "Command" jetzt ausgeführt wird
fn identify<'a>(parts: &'a [&'a str]) -> Command<'a> {
    if parts.is_empty() {
        return Command::Empty;
    }

    if parts[0] == "exit" {
        return Command::Exit;
    }

    Command::Other(parts[0], &parts[1..])
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
            Command::Other(name, args) => {
                println!("Command: {:?}\nArgs: {:?}", name, args)
            }
        }
    }
}
