// Starte des Moduls 'file managment'
// Hier werden die Dateien des Moduls und was nach außen sichtbar ist verarbeitet
mod command;
mod fm;

pub use fm::run;

#[derive(PartialEq)]
pub enum Dialog {
    Nothing,
    ConfirmClose,
    ConfirmDelete,
    Rename,
}

pub struct Editor {
    pub file_path: String,
    pub lines: Vec<Vec<char>>,
    pub cursor_col: usize,
    pub cursor_row: usize,
    pub scroll_row: usize,
    pub modified: bool,
    pub status_message: String,
    pub dialog: Dialog,
    pub file_deleted: bool,
    pub clipboard_line: Option<Vec<char>>,
    pub rename_input: String,
}
