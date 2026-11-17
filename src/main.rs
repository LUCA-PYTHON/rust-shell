use std::env;
use std::io::{self, Write};

use parser::{Command, identify, parse_line};

mod parser;

fn build_promt() -> String {
    match env::current_dir() {
        Ok(path) => format!("rust-shell: <{}>", path.display()),
        Err(..) => String::from("rust-shell> "),
    }
}

fn main() {
    let mut out = io::stdout();

    loop {
        print!("{}", build_promt());
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
            Command::Cd(args) => match args.first() {
                Some(path) => match env::set_current_dir(path) {
                    Ok(()) => {}
                    Err(e) => eprintln!("cd: {}: {}", path, e),
                },
                None => eprintln!("cd: missing argument"),
            },
            Command::Other(name, args) => {
                println!("Command: {:?}\nArgs: {:?}", name, args)
            }
        }
    }
}
