// Befehle für das File Managment
// 'super' nur in Eltern Modul sichtbar (fm.rs)
use super::{Dialog, Editor};
use std::fs;
use std::path::Path;

use crossterm::event::KeyCode;

impl Editor {
    pub(super) fn manage_delete(&mut self, submited: bool) -> bool {
        match self.dialog {
            Dialog::ConfirmClose if submited => true,
            Dialog::ConfirmDelete if submited => match fs::remove_file(&self.file_path) {
                Ok(()) => {
                    self.file_deleted = true;
                    true
                }
                Err(e) => {
                    self.status_message = format!("Error deleting: {}", e);
                    false
                }
            },

            _ => false,
        }
    }

    pub(super) fn handle_rename_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => self.rename_file(),

            KeyCode::Esc => {
                self.dialog = Dialog::Nothing;
                self.rename_input.clear();
            }

            KeyCode::Backspace => {
                self.rename_input.pop();
            }

            KeyCode::Char(c) => {
                self.rename_input.push(c);
            }

            _ => {}
        }

        if self.dialog == Dialog::Rename {
            self.status_message = format!("Rename: {}", self.rename_input);
        }
    }

    pub(super) fn insert_char(&mut self, typed_char: char) {
        self.lines[self.cursor_row].insert(self.cursor_col, typed_char);
        self.cursor_col += 1;
        self.modified = true;
    }

    pub(super) fn insert_new_line(&mut self) {
        let text_after_cursor = self.lines[self.cursor_row].split_off(self.cursor_col);
        self.lines.insert(self.cursor_row + 1, text_after_cursor);
        self.cursor_row += 1;
        self.cursor_col = 0;
        self.modified = true;
    }

    pub(super) fn insert_tab(&mut self) {
        for _ in 0..4 {
            self.insert_char(' ');
        }
    }

    pub(super) fn backspace(&mut self) {
        if self.cursor_col > 0 {
            self.lines[self.cursor_row].remove(self.cursor_col - 1);
            self.cursor_col -= 1;
            self.modified = true;
        } else if self.cursor_row > 0 {
            let removed_line = self.lines.remove(self.cursor_row);
            self.cursor_row -= 1;
            self.cursor_col = self.lines[self.cursor_row].len();
            self.lines[self.cursor_row].extend(removed_line);
            self.modified = true;
        }
    }

    pub(super) fn delete_forward(&mut self) {
        if self.cursor_col < self.lines[self.cursor_row].len() {
            self.lines[self.cursor_row].remove(self.cursor_col);
            self.modified = true;
        } else if self.cursor_row + 1 < self.lines.len() {
            let next = self.lines.remove(self.cursor_row + 1);
            self.lines[self.cursor_row].extend(next);
            self.modified = true;
        }
    }

    pub(super) fn close(&mut self) -> bool {
        if self.modified {
            self.dialog = Dialog::ConfirmClose;
            self.status_message = "Close without saving? (y/n)".to_string();
            false
        } else {
            true
        }
    }

    pub(super) fn save(&mut self) {
        let line_strings: Vec<String> = self
            .lines
            .iter()
            .map(|line| line.iter().collect())
            .collect();
        let file_text = line_strings.join("\n") + "\n";

        match fs::write(&self.file_path, file_text) {
            Ok(()) => {
                self.modified = false;
                self.status_message = format!("Saved {} lines", self.lines.len());
            }
            Err(e) => {
                self.status_message = format!("Error saving: {}", e);
            }
        }
    }

    pub(super) fn delete_file(&mut self) {
        if Path::new(&self.file_path).exists() {
            self.dialog = Dialog::ConfirmDelete;
            self.status_message = "Delete file? (y/n)".to_string();
        } else {
            self.status_message = "Error: file not saved yet, nothing to delete".to_string();
        }
    }

    pub(super) fn delete_row(&mut self) {
        self.lines.remove(self.cursor_row);
        if self.lines.is_empty() {
            self.lines.push(Vec::new());
        }
        self.cursor_row = self.cursor_row.min(self.lines.len() - 1);
        self.modified = true;
    }

    pub(super) fn cut_row(&mut self) {
        self.clipboard_line = Some(self.lines.remove(self.cursor_row));
        if self.lines.is_empty() {
            self.lines.push(Vec::new());
        }
        self.cursor_row = self.cursor_row.min(self.lines.len() - 1);
        self.modified = true;
        self.status_message = "Line cut".to_string();
    }

    pub(super) fn copy_fom_clipboard(&mut self) {
        match &self.clipboard_line {
            Some(copied_line) => {
                self.lines.insert(self.cursor_row, copied_line.clone());
                self.cursor_row += 1;
                self.modified = true;
            }
            None => self.status_message = "Clipboard is empty".to_string(),
        }
    }

    pub(super) fn start_rename(&mut self) {
        self.rename_input = Path::new(&self.file_path)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        self.dialog = Dialog::Rename;
        self.status_message = format!("Rename: {}", self.rename_input);
    }

    pub(super) fn rename_file(&mut self) {
        let new_name = self.rename_input.trim().to_string();
        if new_name.is_empty()
            || new_name == "."
            || new_name == ".."
            || new_name.contains("/")
            || new_name.contains("\\")
        {
            self.status_message = "Error: invalid filename".to_string();
            return;
        }

        let old_path = Path::new(&self.file_path);
        let new_path = old_path.with_file_name(&new_name);
        if new_path == old_path {
            self.dialog = Dialog::Nothing;
            self.status_message = "Filename unchaned".to_string();
            return;
        }
        if new_path.exists() {
            self.status_message = "Error: file already exists".to_string();
            return;
        }

        match fs::rename(old_path, &new_path) {
            Ok(()) => {
                self.file_path = new_path.to_string_lossy().into_owned();
                self.dialog = Dialog::Nothing;
                self.status_message = "File renamed successfully".to_string();
            }
            Err(e) => {
                self.status_message = format!("Error renaming file: {}", e);
            }
        }
    }

    pub(super) fn left(&mut self) {
        self.cursor_col = self.cursor_col.saturating_sub(1);
    }

    pub(super) fn right(&mut self) {
        if self.cursor_col < self.lines[self.cursor_row].len() {
            self.cursor_col += 1;
        }
    }

    pub(super) fn up(&mut self) {
        self.cursor_row = self.cursor_row.saturating_sub(1);
    }

    pub(super) fn down(&mut self) {
        if self.cursor_row + 1 < self.lines.len() {
            self.cursor_row += 1;
        }
    }

    pub(super) fn home(&mut self) {
        self.cursor_col = 0;
    }

    pub(super) fn end(&mut self) {
        self.cursor_col = self.lines[self.cursor_row].len();
    }

    pub(super) fn page_up(&mut self, visible_rows: usize) {
        self.cursor_row = self.cursor_row.saturating_sub(visible_rows);
    }

    pub(super) fn page_down(&mut self, visible_rows: usize) {
        self.cursor_row = self
            .cursor_row
            .saturating_add(visible_rows)
            .min(self.lines.len().saturating_sub(1));
    }

    pub(super) fn no_match(&mut self) {
        self.cursor_col = self.cursor_col.min(self.lines[self.cursor_row].len());
    }
}
