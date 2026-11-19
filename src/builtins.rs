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
