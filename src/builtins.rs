use crate::state::ShellState;
use std::env;

pub fn echo(args: &[&str]) {
    println!("{}", args.join(" "))
}

pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => eprintln!("error: {}", e),
    }
}

pub fn cd(args: &[&str]) -> Result<(), String> {
    match args.first() {
        Some(path) => match env::set_current_dir(path) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!("cd: {}: {}", path, e)),
        },
        None => Err(String::from("cd: missing argument")),
    }
}

pub fn export(args: &[&str], state: &mut ShellState) -> Result<(), String> {
    match args.first() {
        Some(arg) => match arg.split_once('=') {
            Some((name, value)) => {
                state.set(name, value);
                Ok(())
            }
            None => Err(String::from("Syntax: export <NAME>=<value>")),
        },
        None => Err(String::from("export: missing argument")),
    }
}

pub fn env_cmd(args: &[&str], state: &ShellState) -> Result<(), String> {
    match args.first() {
        Some(name) => match state.get(name) {
            Some(value) => {
                println!("{}={}", name, value);
                Ok(())
            }
            None => Err(format!("{}: not set", name)),
        },
        None => {
            for (name, value) in state.all() {
                println! ("{}={}", name, value);
            }
            Ok(())
        }
    }
}

pub fn help() {
    println!("Info Menu for the rust-shell.");
    println!("Befehle:");
    println!("  - echo <text>");
    println!("  - pwd");
    println!("  - cd <path>");
    println!("  - export <variable>=<value>");
}
