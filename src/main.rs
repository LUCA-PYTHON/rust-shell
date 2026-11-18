use ::std::env;
use std::io::{self, Write};

use parser::{Command, identify, parse_line};

mod builtins;
mod parser;
mod state;

fn build_promt() -> String {
    match env::current_dir() {
        Ok(path) => format!("rust-shell: <{}>", path.display()),
        Err(..) => String::from("rust-shell> "),
    }
}

fn main() {
    let mut out = io::stdout();
    let state = state::ShellState::new();
    print!("{:?}", state.get("HOME"));

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
            Command::Echo(args) => builtins::echo(args),
            Command::Pwd => builtins::pwd(),
            Command::Cd(args) => builtins::cd(args),
            Command::Other(name, args) => {
                println!("Command: {:?}\nArgs: {:?}", name, args)
            }
        }
    }
}
