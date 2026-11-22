use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Attribute, Print, SetAttribute},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::{self, Write};

pub fn run(args: &[&str]) -> Result<(), String> {
    let path = *args.first().ok_or("fm: missing argument")?;
    let text = fs::read_to_string(path).map_err(|e| format!("fm: {} : {}", path, e))?;

    let mut lines: Vec<Vec<char>> = text
        .lines()
        .map(|l| l.replace('\t', "    ").chars().collect())
        .collect();
    if lines.is_empty() {
        lines.push(Vec::new());
    }

    let (mut cx, mut cy) = (0usize, 0usize);
    let mut changed = false;
    let mut msg = String::new();

    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    loop {
        draw(path, &lines, cx, cy, changed, &msg);

        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            msg.clear();
            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
            if ctrl && matches!(key.code, KeyCode::Char('x') | KeyCode::Char('c')) {
                break;
            }

            let alt = key.modifiers.contains(KeyModifiers::ALT);

            match key.code {
                KeyCode::Char('s') if ctrl => {
                    let strings: Vec<String> = lines.iter().map(|l| l.iter().collect()).collect();
                    let content = strings.join("\n") + "\n";

                    match fs::write(path, content) {
                        Ok(()) => {
                            changed = false;
                            msg = format!("Saved {} lines", lines.len());
                        }
                        Err(e) => {
                            msg = format!("Error saving: {}", e);
                        }
                    }
                }
                KeyCode::Enter => {
                    let rest = lines[cy].split_off(cx);
                    lines.insert(cy + 1, rest);
                    cy += 1;
                    cx = 0;
                    changed = true;
                }
                KeyCode::Backspace => {
                    if cx > 0 {
                        lines[cy].remove(cx - 1);
                        cx -= 1;
                        changed = true;
                    } else if cy > 0 {
                        let line = lines.remove(cy);
                        cy -= 1;
                        cx = lines[cy].len();
                        lines[cy].extend(line);
                        changed = true;
                    }
                }
                KeyCode::Char(c) if !ctrl || alt => {
                    lines[cy].insert(cx, c);
                    cx += 1;
                    changed = true;
                }
                KeyCode::Left => {
                    if cx > 0 {
                        cx -= cx.saturating_sub(1);
                    }
                }
                KeyCode::Right => {
                    if cx < lines[cy].len() {
                        cx += 1;
                    }
                }
                KeyCode::Up => {
                    if cy > 0 {
                        cy -= cy.saturating_sub(1);
                    }
                }
                KeyCode::Down if cy + 1 < lines.len() => {
                    cy += 1;
                }
                _ => {}
            }
            cx = cx.min(lines[cy].len());
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

fn draw(path: &str, lines: &[Vec<char>], cx: usize, cy: usize, changed: bool, msg: &str) {
    let mut out = io::stdout();
    let (width, height) = terminal::size().unwrap();
    let (width, height) = (width as usize, height as usize);

    queue!(out, Clear(ClearType::All)).unwrap();
    queue!(
        out,
        MoveTo(0, 0),
        SetAttribute(Attribute::Reverse),
        Print(bar(
            &format!(" fm {}{}", path, if changed { "  [modified]" } else { "" }),
            width
        )),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();

    for (i, line) in lines.iter().enumerate().take(height.saturating_sub(2)) {
        let visible: String = line.iter().take(width).collect();
        queue!(out, MoveTo(0, i as u16 + 1), Print(visible)).unwrap();
    }

    let info = if msg.is_empty() {
        format!("Line {}, Col {}", cy + 1, cx + 1)
    } else {
        msg.to_string()
    };

    queue!(
        out,
        MoveTo(0, height.saturating_sub(1) as u16),
        SetAttribute(Attribute::Reverse),
        Print(" CTRL+X "),
        SetAttribute(Attribute::Reset),
        Print(" Close  "),
        SetAttribute(Attribute::Reverse),
        Print(" CTRL+S "),
        SetAttribute(Attribute::Reset),
        Print(format!(" Save    {}", info))
    )
    .unwrap();

    let cur_x = cx.min(width.saturating_sub(1));
    let cur_y = (cy + 1).min(height.saturating_sub(2));
    queue!(out, MoveTo(cur_x as u16, cur_y as u16)).unwrap();

    out.flush().unwrap();
}
