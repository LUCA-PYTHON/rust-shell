use crate::state::ShellState;
use std::env;
use std::io::Write;

type Res = Result<(), String>;

fn io_err(e: std::io::Error) -> String {
    e.to_string()
}

pub fn echo(args: &[&str], out: &mut impl Write) -> Res {
    writeln!(out, "{}", args.join(" ")).map_err(io_err)
}

pub fn pwd(out: &mut impl Write) -> Res {
    let path = env::current_dir().map_err(|e| format!("pwd: {}", e))?;
    writeln!(out, "{}", path.display()).map_err(io_err)
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

pub fn env_cmd(args: &[&str], state: &ShellState, out: &mut impl Write) -> Res {
    match args.first() {
        Some(name) => match state.get(name) {
            Some(value) => writeln!(out, "{}={}", name, value).map_err(io_err),
            None => Err(format!("{}: not set", name)),
        },
        None => {
            for (name, value) in state.all() {
                writeln!(out, "{}={}", name, value).map_err(io_err)?;
            }
            Ok(())
        }
    }
}

pub fn help(out: &mut impl Write) -> Res {
    let text = "\
Info Menu for the rust-shell.
Befehle:
  -> echo <text>
  -> pwd
  -> cd <path>
  -> export <name>=<value>
  -> env <NAME>
  -> fm <path>
  -> help                         Hilfe Menu anzeigen
";
    write!(out, "{}", text).map_err(io_err)
}
