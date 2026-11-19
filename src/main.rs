use rust_shell::builtins;
use rust_shell::parser::{Command, identify, parse_line};
use rust_shell::state::{ShellState, expand};

use std::env;
use std::io::{self, Write};

fn build_promt() -> String {
    match env::current_dir() {
        Ok(path) => format!("rust-shell: {}>", path.display()),
        Err(..) => String::from("rust-shell>"),
    }
}

fn main() {
    let mut out = io::stdout();
    let state = ShellState::new();

    loop {
        print!("{} ", build_promt());
        out.flush().unwrap();
        let mut text = String::new();
        let bytes = io::stdin().read_line(&mut text).unwrap();
        if bytes == 0 {
            break;
        }

        let line = text.trim();
        let parts = parse_line(line);
        let expanded: Vec<String> = parts.iter().map(|w| expand(w, &state)).collect();
        let refs: Vec<&str> = expanded.iter().map(|s| s.as_str()).collect();

        match identify(&refs) {
            Command::Exit => break,
            Command::Empty => continue,
            Command::Echo(args) => builtins::echo(args),
            Command::Pwd => builtins::pwd(),
            Command::Cd(args) => builtins::cd(args),
            Command::Other(name, args) => {
                println!("Command: {:?}\nArgs: {:?}", name, args)
            }
        }
    }
}
