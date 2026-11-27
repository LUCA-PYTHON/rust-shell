// Starte des Moduls 'file managment'
// Hier werden die Dateien des Moduls und was nach außen sichtbar ist verarbeitet
mod command;
mod fm;

pub use fm::run;

#[derive(PartialEq)]
enum Dialog {
    Nothing,
    ConfirmClose,
    ConfirmDelete,
    Rename,
}

struct Editor {
    file_path: String,
    lines: Vec<Vec<char>>,
    cursor_col: usize,
    cursor_row: usize,
    scroll_row: usize,
    modified: bool,
    status_message: String,
    dialog: Dialog,
    file_deleted: bool,
    clipboard_line: Option<Vec<char>>,
    rename_input: String,
}
