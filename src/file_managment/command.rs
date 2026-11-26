// Befehle für das File Managment
// 'super' nur in Eltern Modul sichtbar (fm.rs)
use super::Editor;

impl Editor {
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
}
