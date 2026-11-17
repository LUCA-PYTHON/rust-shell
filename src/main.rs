use std::env;
use std::io::{self, Write};

use parser::{identify, parse_line, Command};

mod parser;

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
            Command::Echo(args) => {
                println!("{}", args.join(" "))
            }
            Command::Pwd => match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("error: {}", e),
            },
            Command::Other(name, args) => {
                println!("Command: {:?}\nArgs: {:?}", name, args)
            }
        }
    }
}
