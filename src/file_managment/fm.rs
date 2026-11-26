// Hauptprogramm des Moduls 'file managment'
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::{Dialog, Editor};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run(args: &[&str]) -> Result<(), String> {
    let file_path = *args.first().ok_or("fm: missing argument")?;
    let is_new_file = !Path::new(file_path).exists();
    let file_content = if is_new_file {
        String::new()
    } else {
        fs::read_to_string(file_path).map_err(|e| format!("fm: {} : {}", file_path, e))?
    };

    let mut lines: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.replace('\t', "    ").chars().collect())
        .collect();
    if lines.is_empty() {
        lines.push(Vec::new());
    }

    let mut editor = Editor {
        file_path: file_path.to_string(),
        lines,
        cursor_col: 0,
        cursor_row: 0,
        scroll_row: 0,
        modified: false,
        status_message: if is_new_file {
            "New File created".to_string()
        } else {
            String::new()
        },
        dialog: Dialog::Nothing,
        file_deleted: false,
        clipboard_line: None,
    };

    terminal::enable_raw_mode().unwrap();
    execute!(io::stdout(), EnterAlternateScreen).unwrap();

    loop {
        let (_, terminal_height) = terminal::size().unwrap();
        let visible_rows = (terminal_height as usize).saturating_sub(2).max(1);
        if editor.cursor_row < editor.scroll_row {
            editor.scroll_row = editor.cursor_row;
        } else if editor.cursor_row >= editor.scroll_row + visible_rows {
            editor.scroll_row = editor.cursor_row + 1 - visible_rows;
        }

        draw(&editor);

        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Release {
                continue;
            }

            editor.status_message.clear();
            if editor.dialog != Dialog::Nothing {
                let submited = key_event.code == KeyCode::Char('y');
                match editor.dialog {
                    Dialog::ConfirmClose if submited => break,
                    Dialog::ConfirmDelete if submited => match fs::remove_file(&editor.file_path) {
                        Ok(()) => {
                            editor.file_deleted = true;
                            break;
                        }
                        Err(e) => editor.status_message = format!("Error deleting: {}", e),
                    },
                    _ => {}
                }
                editor.dialog = Dialog::Nothing;
                continue;
            }

            let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
            let alt = key_event.modifiers.contains(KeyModifiers::ALT);

            match key_event.code {
                KeyCode::Char('x') | KeyCode::Char('c') if ctrl => {
                    if editor.modified {
                        editor.dialog = Dialog::ConfirmClose;
                        editor.status_message = "Close without saving? (y/n)".to_string();
                    } else {
                        break;
                    }
                }
                KeyCode::Char('s') if ctrl => {
                    let line_strings: Vec<String> = editor
                        .lines
                        .iter()
                        .map(|line| line.iter().collect())
                        .collect();
                    let file_text = line_strings.join("\n") + "\n";

                    match fs::write(&editor.file_path, file_text) {
                        Ok(()) => {
                            editor.modified = false;
                            editor.status_message = format!("Saved {} lines", editor.lines.len());
                        }
                        Err(e) => {
                            editor.status_message = format!("Error saving: {}", e);
                        }
                    }
                }
                KeyCode::Char('d') if ctrl => {
                    if Path::new(&editor.file_path).exists() {
                        editor.dialog = Dialog::ConfirmDelete;
                        editor.status_message = "Delete file? (y/n)".to_string();
                    } else {
                        editor.status_message =
                            "Error: file not saved yet, nothing to delete".to_string();
                    }
                }
                KeyCode::Char('l') if ctrl => {
                    editor.lines.remove(editor.cursor_row);
                    if editor.lines.is_empty() {
                        editor.lines.push(Vec::new());
                    }
                    editor.cursor_row = editor.cursor_row.min(editor.lines.len() - 1);
                    editor.modified = true;
                }
                KeyCode::Char('k') if ctrl => {
                    editor.clipboard_line = Some(editor.lines.remove(editor.cursor_row));
                    if editor.lines.is_empty() {
                        editor.lines.push(Vec::new());
                    }
                    editor.cursor_row = editor.cursor_row.min(editor.lines.len() - 1);
                    editor.modified = true;
                    editor.status_message = "Line cut".to_string();
                }
                KeyCode::Char('u') if ctrl => match &editor.clipboard_line {
                    Some(copied_line) => {
                        editor.lines.insert(editor.cursor_row, copied_line.clone());
                        editor.cursor_row += 1;
                        editor.modified = true;
                    }
                    None => editor.status_message = "Clipboard is empty".to_string(),
                },
                KeyCode::Enter => editor.insert_new_line(),
                KeyCode::Backspace => editor.backspace(),
                KeyCode::Char(c) if !ctrl || alt => editor.insert_char(c),
                KeyCode::Left => {
                    editor.cursor_col = editor.cursor_col.saturating_sub(1);
                }
                KeyCode::Right => {
                    if editor.cursor_col < editor.lines[editor.cursor_row].len() {
                        editor.cursor_col += 1;
                    }
                }
                KeyCode::Up => {
                    editor.cursor_row = editor.cursor_row.saturating_sub(1);
                }
                KeyCode::Down if editor.cursor_row + 1 < editor.lines.len() => {
                    editor.cursor_row += 1;
                }
                KeyCode::Home => {
                    editor.cursor_col = 0;
                }
                KeyCode::End => {
                    editor.cursor_col = editor.lines[editor.cursor_row].len();
                }
                KeyCode::Delete => editor.delete_forward(),
                KeyCode::PageUp => {
                    editor.cursor_row = editor.cursor_row.saturating_sub(visible_rows);
                }
                KeyCode::PageDown => {
                    editor.cursor_row =
                        (editor.cursor_row + visible_rows).min(editor.lines.len() - 1);
                }
                KeyCode::Tab => editor.insert_tab(),
                _ => {}
            }
            editor.cursor_col = editor.cursor_col.min(editor.lines[editor.cursor_row].len());
        }
    }

    execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    terminal::disable_raw_mode().unwrap();

    if editor.file_deleted {
        println!("fm: {} deleted", editor.file_path);
    }

    Ok(())
}

// Text auf Terminal Breite abschneiden
fn bar(text: &str, width: usize) -> String {
    let shortened: String = text.chars().take(width).collect();
    format!("{:<width$}", shortened, width = width)
}

// Zeile mit links Zeilennummer und rechts Dateitext
fn h_bar(left_text: &str, right_text: &str, width: usize) -> String {
    let right_text_len = right_text.chars().count();
    let left_text_cut: String = left_text
        .chars()
        .take(width.saturating_sub(right_text_len + 1))
        .collect();
    let padding = width.saturating_sub(left_text_cut.chars().count() + right_text_len);
    format!("{}{}{}", left_text_cut, " ".repeat(padding), right_text)
}

// Datei 'zeichen', header -> dateiinhalt -> footer
fn draw(editor: &Editor) {
    let mut stdout = io::stdout();
    let (width, height) = terminal::size().unwrap();
    let (width, height) = (width as usize, height as usize);
    let position_text = format!(
        "Line {}, Col {}",
        editor.cursor_row + 1,
        editor.cursor_col + 1
    );
    let title_text = format!(
        " fm {}{}",
        editor.file_path,
        if editor.modified { " [modified]" } else { "" }
    );

    let number_width = editor.lines.len().to_string().len();
    let gutter_width = number_width + 3;
    let text_width = width.saturating_sub(gutter_width);

    queue!(stdout, Clear(ClearType::All)).unwrap();
    queue!(
        stdout,
        MoveTo(0, 0),
        SetAttribute(Attribute::Reverse),
        Print(h_bar(&title_text, &position_text, width)),
        SetAttribute(Attribute::Reset)
    )
    .unwrap();

    for (screen_row, line) in editor
        .lines
        .iter()
        .skip(editor.scroll_row)
        .enumerate()
        .take(height.saturating_sub(2))
    {
        let visible_text: String = line.iter().take(text_width).collect();
        queue!(
            stdout,
            MoveTo(0, screen_row as u16 + 1),
            SetForegroundColor(Color::DarkGrey),
            Print(format!(
                "{:>width$} | ",
                editor.scroll_row + screen_row + 1,
                width = number_width
            )),
            ResetColor,
            Print(visible_text)
        )
        .unwrap();
    }

    let footer_row = height.saturating_sub(1) as u16;

    if editor.status_message.is_empty() {
        queue!(
            stdout,
            MoveTo(0, footer_row),
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
            Print(" Delete line "),
            SetAttribute(Attribute::Reverse),
            Print(" CTRL+K "),
            SetAttribute(Attribute::Reset),
            Print(" Cut line "),
            SetAttribute(Attribute::Reverse),
            Print(" CTRL+U "),
            SetAttribute(Attribute::Reset),
            Print(" Paste line ")
        )
        .unwrap();
    } else {
        let message_color =
            if editor.status_message.starts_with("Error") || editor.status_message.contains('?') {
                Color::DarkRed
            } else {
                Color::DarkGreen
            };
        queue!(
            stdout,
            MoveTo(0, footer_row),
            SetBackgroundColor(message_color),
            SetForegroundColor(Color::White),
            Print(bar(&format!(" {}", editor.status_message), width)),
            ResetColor
        )
        .unwrap();
    }

    let screen_x = (gutter_width + editor.cursor_col).min(width.saturating_sub(1));
    let screen_y = (editor.cursor_row - editor.scroll_row + 1).min(height.saturating_sub(2));
    queue!(stdout, MoveTo(screen_x as u16, screen_y as u16)).unwrap();

    stdout.flush().unwrap();
}
