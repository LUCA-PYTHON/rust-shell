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

pub fn cd(args: &[&str]) {
    match args.first() {
        Some(path) => match env::set_current_dir(path) {
            Ok(()) => {}
            Err(e) => eprintln!("cd: {}: {}", path, e),
        },
        None => eprintln!("cd: missing argument"),
    }
}
