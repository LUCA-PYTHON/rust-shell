use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(PartialEq)]
enum Dialog {
    Nothing,
    ConfirmClose,
    ConfirmDelete,
}

pub fn run(args: &[&str]) -> Result<(), String> {
    let path = *args.first().ok_or("fm: missing argument")?;
    let is_new = !Path::new(path).exists();
    let text = if is_new {
        String::new()
    } else {
        fs::read_to_string(path).map_err(|e| format!("fm: {} : {}", path, e))?
    };

    let mut lines: Vec<Vec<char>> = text
        .lines()
        .map(|l| l.replace('\t', "    ").chars().collect())
        .collect();
    if lines.is_empty() {
        lines.push(Vec::new());
    }

    let (mut cx, mut cy) = (0usize, 0usize);
    let mut top = 0usize;
    let mut changed = false;
    let mut msg = if is_new {
        "New File created".to_string()
    } else {
        String::new()
    };

    let mut dialog = Dialog::Nothing;
    let mut deleted = false;

    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    loop {
        let (_, h) = terminal::size().unwrap();
        let rows = (h as usize).saturating_sub(2).max(1);
        if cy < top {
            top = cy;
        } else if cy >= top + rows {
            top = cy + 1 - rows;
        }

        draw(path, &lines, cx, cy, top, changed, &msg);

        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Release {
                continue;
            }

            msg.clear();
            if dialog != Dialog::Nothing {
                let submit = key.code == KeyCode::Char('y');
                match dialog {
                    Dialog::ConfirmClose if submit => break,
                    Dialog::ConfirmDelete if submit => match fs::remove_file(path) {
                        Ok(()) => {
                            deleted = true;
                            break;
                        }
                        Err(e) => msg = format!("Error deleting: {}", e),
                    },
                    _ => {}
                }
                dialog = Dialog::Nothing;
                continue;
            }

            let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
            let alt = key.modifiers.contains(KeyModifiers::ALT);

            match key.code {
                KeyCode::Char('x') | KeyCode::Char('c') if ctrl => {
                    if changed {
                        dialog = Dialog::ConfirmClose;
                        msg = "Close without saving? (y/n)".to_string();
                    } else {
                        break;
                    }
                }
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
                KeyCode::Char('d') if ctrl => {
                    if Path::new(path).exists() {
                        dialog = Dialog::ConfirmDelete;
                        msg = "Delete file? (y/n)".to_string();
                    } else {
                        msg = "Error: file not saved yet, nothing to delete".to_string();
                    }
                }
                KeyCode::Char('l') if ctrl => {
                    lines.remove(cy);
                    if lines.is_empty() {
                        lines.push(Vec::new());
                    }
                    cy = cy.min(lines.len() - 1);
                    changed = true;
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
                    cx = cx.saturating_sub(1);
                }
                KeyCode::Right => {
                    if cx < lines[cy].len() {
                        cx += 1;
                    }
                }
                KeyCode::Up => {
                    cy = cy.saturating_sub(1);
                }
                KeyCode::Down if cy + 1 < lines.len() => {
                    cy += 1;
                }
                KeyCode::Home => {
                    cx = 0;
                }
                KeyCode::End => {
                    cx = lines[cy].len();
                }
                KeyCode::Delete => {
                    if cx < lines[cy].len() {
                        lines[cy].remove(cx);
                        changed = true;
                    } else if cy + 1 < lines.len() {
                        let next = lines.remove(cy + 1);
                        lines[cy].extend(next);
                        changed = true;
                    }
                }
                KeyCode::PageUp => {
                    cy = cy.saturating_sub(rows);
                }
                KeyCode::PageDown => {
                    cy = (cy + rows).min(lines.len() - 1);
                }
                KeyCode::Tab => {
                    for _ in 0..4 {
                        lines[cy].insert(cx, ' ');
                        cx += 1;
                    }
                    changed == true;
                }
                _ => {}
            }
            cx = cx.min(lines[cy].len());
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();

    if deleted {
        println!("fm: {} deleted", path);
    }

    Ok(())
}

fn bar(text: &str, width: usize) -> String {
    let cut: String = text.chars().take(width).collect();
    format!("{:<width$}", cut, width = width)
}

fn h_bar(left: &str, right: &str, width: usize) -> String {
    let right_len = right.chars().count();
    let left_cut: String = left
        .chars()
        .take(width.saturating_sub(right_len + 1))
        .collect();
    let pad = width.saturating_sub(left_cut.chars().count() + right_len);
    format!("{}{}{}", left_cut, " ".repeat(pad), right)
}

fn draw(
    path: &str,
    lines: &[Vec<char>],
    cx: usize,
    cy: usize,
    top: usize,
    changed: bool,
    msg: &str,
) {
    let mut out = io::stdout();
    let (width, height) = terminal::size().unwrap();
    let (width, height) = (width as usize, height as usize);
    let pos = format!("Line {}, Col {}", cy + 1, cx + 1);
    let title = format!(" fm {}{}", path, if changed { " [modified]" } else { "" });

    queue!(out, Clear(ClearType::All)).unwrap();
    queue!(
        out,
        MoveTo(0, 0),
        SetAttribute(Attribute::Reverse),
        Print(h_bar(&title, &pos, width)),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();

    for (i, line) in lines
        .iter()
        .skip(top)
        .enumerate()
        .take(height.saturating_sub(2))
    {
        let visible: String = line.iter().take(width).collect();
        queue!(out, MoveTo(0, i as u16 + 1), Print(visible)).unwrap();
    }

    let last = height.saturating_sub(1) as u16;

    if msg.is_empty() {
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
            Print(" Save "),
            SetAttribute(Attribute::Reverse),
            Print(" CTRL+D "),
            SetAttribute(Attribute::Reset),
            Print(" Delete "),
            SetAttribute(Attribute::Reverse),
            Print(" CTRL+L "),
            SetAttribute(Attribute::Reset),
            Print(" Delete line".to_string())
        )
        .unwrap();
    } else {
        let color = if msg.starts_with("Error") || msg.contains('?') {
            Color::DarkRed
        } else {
            Color::DarkGreen
        };
        queue!(
            out,
            MoveTo(0, last),
            SetBackgroundColor(color),
            SetForegroundColor(Color::White),
            Print(bar(&format!(" {}", msg), width)),
            ResetColor
        )
        .unwrap();
    }

    let cur_x = cx.min(width.saturating_sub(1));
    let cur_y = (cy - top + 1).min(height.saturating_sub(2));
    queue!(out, MoveTo(cur_x as u16, cur_y as u16)).unwrap();

    out.flush().unwrap();
}
