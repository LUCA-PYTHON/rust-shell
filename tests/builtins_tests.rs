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
    assert!(matches!(result, Command::Cd(args) if args == ["/tmp"]))
}