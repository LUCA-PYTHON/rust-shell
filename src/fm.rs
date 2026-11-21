use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::{self, Write};

pub fn run(args: &[&str]) -> Result<(), String> {
    let path = *args.first().ok_or("fm: missing argument")?;
    let text = fs::read_to_string(path).map_err(|e| format!("fm: {} : {}", path, e))?;

    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    loop {
        draw(path, &text);

        if let Event::Key(key) = event::read().unwrap() {
            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
            if ctrl && matches!(key.code, KeyCode::Char('x') | KeyCode::Char('c')) {
                break;
            }
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();
    Ok(())
}

fn bar(text: &str, width: usize) -> String {
    let cut: String = text.chars().take(width).collect();
    format!("{:<width$}", cut, width = width)
}

fn draw(path: &str, text: &str) {
    let mut out = io::stdout();
    let (width, height) = terminal::size().unwrap();
    let (width, height) = (width as usize, height as usize);

    queue!(out, Clear(ClearType::All)).unwrap();
    queue!(
        out,
        MoveTo(0, 0),
        SetAttribute(Attribute::Reverse),
        Print(bar(&format!(" fm {}", path), width)),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();

    for (i, line) in text.lines().enumerate().take(height.saturating_sub(2)) {
        let visible: String = line.chars().take(width).collect();
        queue!(out, MoveTo(0, i as u16 + 1), Print(visible)).unwrap();
    }

    queue!(
        out,
        MoveTo(0, height.saturating_sub(1) as u16),
        SetAttribute(Attribute::Reverse),
        Print(" CTRL+X "),
        SetAttribute(Attribute::Reset),
        Print(" Close")
    )
    .unwrap();

    out.flush().unwrap();
}
