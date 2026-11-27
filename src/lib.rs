pub mod builtins;
pub mod file_managment;
pub mod parser;
pub mod state;

use parser::{Command, identify, parse_line};
use state::{ShellState, expand};
use std::io::Write;

pub enum Flow {
    Continue,
    Exit,
}

pub fn execute(line: &str, state: &mut ShellState, out: &mut impl Write) -> Flow {
    let parts = parse_line(line.trim());
    let expanded: Vec<String> = parts.iter().map(|w| expand(w, state)).collect();
    let refs: Vec<&str> = expanded.iter().map(|s| s.as_str()).collect();

    let result = match identify(&refs) {
        Command::Exit => return Flow::Exit,
        Command::Empty => Ok(()),
        Command::Echo(args) => builtins::echo(args, out),
        Command::Pwd => builtins::pwd(out),
        Command::Cd(args) => builtins::cd(args),
        Command::Export(args) => builtins::export(args, state),
        Command::Env(args) => builtins::env_cmd(args, state, out),
        Command::Fm(args) => file_managment::run(args),
        Command::Help => builtins::help(out),
        Command::Other(name, args) => {
            writeln!(out, "Command: {:?}\nArgs: {:?}", name, args).map_err(|e| e.to_string())
        }
    };
    if let Err(e) = result {
        let _ = writeln!(out, "{}", e);
    }
    Flow::Continue
}
