use std::io::{self, Write};

fn main()
{
    let mut out = io::stdout();

    loop
    {
        print!("rust-shell> ");
        out.flush().unwrap();
        let mut text = String::new();
        io::stdin().read_line(&mut text).unwrap();

        let clean = text.trim();
        if clean.is_empty()
        {
            continue;
        }
        if clean == "exit"
        {
            break;
        }
        println!("You typed: '{}'", clean);
    }
}