// Enumeration, Datentyp mit mehreren Varianten eines Werts
pub enum Command<'a> {
    Exit,
    Empty,
    Echo(&'a [&'a str]),
    Pwd,
    Cd(&'a [&'a str]),
    Other(&'a str, &'a [&'a str]),
}

// Rausfiltern welcher "Command" jetzt ausgeführt wird
pub fn identify<'a>(parts: &'a [&'a str]) -> Command<'a> {
    if parts.is_empty() {
        return Command::Empty;
    }

    if parts[0] == "exit" {
        return Command::Exit;
    }

    if parts[0] == "echo" {
        return Command::Echo(&parts[1..]);
    }

    if parts[0] == "pwd" {
        return Command::Pwd;
    }

    if parts[0] == "cd" {
        return Command::Cd(&parts[1..]);
    }

    Command::Other(parts[0], &parts[1..])
}

pub fn parse_line(line: &str) -> Vec<&str> {
    line.split_whitespace().collect()
}
