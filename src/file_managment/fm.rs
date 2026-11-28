// Hauptprogramm des Moduls 'file managment'
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyEventKind},
    execute, queue,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use super::Editor;
use std::io::{self, Write};

struct TerminalGuard {
    raw_mode_enabled: bool,
    alternate_screen_enabled: bool,
}

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        terminal::enable_raw_mode()?;

        let mut guard = Self {
            raw_mode_enabled: true,
            alternate_screen_enabled: false,
        };

        guard.alternate_screen_enabled = true;
        execute!(io::stdout(), EnterAlternateScreen)?;

        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.alternate_screen_enabled {
            let _ = execute!(io::stdout(), LeaveAlternateScreen);
        }

        if self.raw_mode_enabled {
            let _ = terminal::disable_raw_mode();
        }
    }
}

pub fn run(args: &[&str]) -> Result<(), String> {
    let file_path = *args.first().ok_or("fm: missing argument")?;
    let mut editor = Editor::open(file_path)?;

    let terminal_guard =
        TerminalGuard::new().map_err(|e| format!("fm: terminal setup failed: {}", e))?;

    loop {
        let (_, terminal_height) = terminal::size().unwrap();
        let visible_rows = (terminal_height as usize).saturating_sub(2).max(1);
        editor.scroll_to_cursor(visible_rows);

        draw(&editor);

        if let Event::Key(key_event) = event::read().unwrap() {
            if key_event.kind == KeyEventKind::Release {
                continue;
            }

            if editor.handle_key(key_event.code, key_event.modifiers, visible_rows) {
                break;
            }
        }
    }

    drop(terminal_guard);

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
            Print(" Paste "),
            SetAttribute(Attribute::Reverse),
            Print(" CTRL+R "),
            SetAttribute(Attribute::Reset),
            Print(" Rename file ")
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
