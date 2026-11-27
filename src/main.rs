use rust_shell::state::ShellState;
use rust_shell::{Flow, execute};

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
    let mut state = ShellState::new();

    loop {
        print!("{} ", build_promt());
        out.flush().unwrap();
        let mut text = String::new();
        if io::stdin().read_line(&mut text).unwrap() == 0 {
            break;
        }

        if let Flow::Exit = execute(&text, &mut state, &mut out) {
            break;
        }
    }
}
