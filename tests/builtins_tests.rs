use rust_shell::builtins;
use rust_shell::state::{ShellState};
use rust_shell::parser::{Command, identify};

#[test]
fn identifies_exit() {
    let result = identify(&["exit"]);
    assert!(matches!(result, Command::Exit));
}

#[test]
fn identifies_pwd() {
    let result = identify(&["pwd"]);
    assert!(matches!(result, Command::Pwd));
}

#[test]
fn identifies_empty() {
    let input: [&str; 0] = [];
    let result = identify(&input);
    assert!(matches!(result, Command::Empty));
}

#[test]
fn identifies_echo_empty() {
    let result = identify(&["echo"]);
    assert!(matches!(result, Command::Echo(args) if args.is_empty()));
}

#[test]
fn identifies_echo_args() {
    let result = identify(&["echo", "a", "b"]);
    assert!(matches!(result, Command::Echo(args) if args == ["a", "b"]));
}

#[test]
fn identifies_cd() {
    let result = identify(&["cd", "/tmp"]);
    assert!(matches!(result, Command::Cd(args) if args == ["/tmp"]));
}

#[test]
fn identifies_cd_empty() {
    let result = identify(&["cd"]);
    assert!(matches!(result, Command::Cd(args) if args.is_empty()));
}

#[test]
fn cd_to_missing_dir_fails() {
    let result = builtins::cd(&["/notexisting"]);
    assert!(result.is_err());
}

#[test]
fn identifies_export() {
    let result = identify(&["export", "TEST=test"]);
    assert!(matches!(result, Command::Export(args) if args == ["TEST=test"]));
}

#[test]
fn export_without_argument_fails() {
    let mut state = ShellState::new();
    let input: [&str; 0] = [];
    assert!(builtins::export(&input, &mut state).is_err());
}

#[test]
fn export_without_equals_sign_fails() {
    let mut state = ShellState::new();
    let input = ["test"];
    assert!(builtins::export(&input, &mut state).is_err());
}

#[test]
#[test]
fn export_with_name_and_value_succeeds() {
    let mut state = ShellState::new();
    let result = builtins::export(&["NAME=testuser"], &mut state);
    assert!(result.is_ok());
    assert_eq!(state.get("NAME"), Some(&String::from("testuser")));
}