// Enumeration, Datentyp mit mehreren Varianten eines Werts
pub enum Command<'a> {
    Exit,
    Empty,
    Echo(&'a [&'a str]),
    Pwd,
    Cd(&'a [&'a str]),
    Export(&'a [&'a str]),
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

    if parts[0] == "export" {
        return Command::Export(&parts[1..]);
    }

    Command::Other(parts[0], &parts[1..])
}

// Eingabe zerlegen und "" erkennen
pub fn parse_line(line: &str) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for c in line.chars() {
        if c == '"' {
            in_quotes = !in_quotes;
        } else if c.is_whitespace() && !in_quotes {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }

    words
}
