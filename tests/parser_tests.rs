use rust_shell::parser::parse_line;

#[test]
fn splits_on_whitespace() {
    assert_eq!(parse_line("ls  -l"), ["ls", "-l"]);
}