#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crossterm::event::{KeyCode, KeyModifiers};
use eframe::egui::{self, Color32};
use rust_shell::file_managment::Editor;
use rust_shell::parser::{Command, identify, parse_line};
use rust_shell::state::{ShellState, expand};
use rust_shell::{Flow, execute};

struct ShellApp {
    state: ShellState,
    input: String,
    output: String,
    fm: Option<Editor>,
    fm_rows: usize,
}

fn build_prompt() -> String {
    match std::env::current_dir() {
        Ok(path) => format!("rust-shell: {}>", path.display()),
        Err(..) => String::from("rust-shell>"),
    }
}

fn fm_args(line: &str, state: &ShellState) -> Option<Vec<String>> {
    let parts = parse_line(line.trim());
    let expanded: Vec<String> = parts.iter().map(|w| expand(w, state)).collect();
    let refs: Vec<&str> = expanded.iter().map(|s| s.as_str()).collect();
    let result = match identify(&refs) {
        Command::Fm(args) => Some(args.iter().map(|s| s.to_string()).collect()),
        _ => None,
    };
    result
}

fn keys_from_event(event: &egui::Event) -> Vec<(KeyCode, KeyModifiers)> {
    use egui::Key;

    match event {
        egui::Event::Text(text) => text
            .chars()
            .map(|c| (KeyCode::Char(c), KeyModifiers::NONE))
            .collect(),
        egui::Event::Cut => vec![(KeyCode::Char('x'), KeyModifiers::CONTROL)],
        egui::Event::Copy => vec![(KeyCode::Char('c'), KeyModifiers::CONTROL)],
        egui::Event::Key {
            key,
            pressed: true,
            modifiers,
            ..
        } => {
            let special = match key {
                Key::Enter => Some(KeyCode::Enter),
                Key::Tab => Some(KeyCode::Tab),
                Key::Backspace => Some(KeyCode::Backspace),
                Key::Delete => Some(KeyCode::Delete),
                Key::Escape => Some(KeyCode::Esc),
                Key::ArrowLeft => Some(KeyCode::Left),
                Key::ArrowRight => Some(KeyCode::Right),
                Key::ArrowUp => Some(KeyCode::Up),
                Key::ArrowDown => Some(KeyCode::Down),
                Key::Home => Some(KeyCode::Home),
                Key::End => Some(KeyCode::End),
                Key::PageUp => Some(KeyCode::PageUp),
                Key::PageDown => Some(KeyCode::PageDown),
                _ => None,
            };
            if let Some(code) = special {
                return vec![(code, KeyModifiers::NONE)];
            }

            if modifiers.ctrl && !modifiers.alt {
                let letter = match key {
                    Key::S => Some('s'),
                    Key::D => Some('d'),
                    Key::L => Some('l'),
                    Key::K => Some('k'),
                    Key::U => Some('u'),
                    Key::R => Some('r'),
                    _ => None,
                };
                if let Some(c) = letter {
                    return vec![(KeyCode::Char(c), KeyModifiers::CONTROL)];
                }
            }
            vec![]
        }
        _ => vec![],
    }
}

impl ShellApp {
    fn run_input(&mut self, prompt: &str, ctx: &egui::Context) {
        let line = std::mem::take(&mut self.input);
        self.output.push_str(&format!("{} {}\n", prompt, line));

        if let Some(args) = fm_args(&line, &self.state) {
            match args.first() {
                None => self.output.push_str("fm: missing argument\n"),
                Some(path) => match Editor::open(path) {
                    Ok(editor) => self.fm = Some(editor),
                    Err(e) => self.output.push_str(&format!("{}\n", e)),
                },
            }
            return;
        }

        let mut buf: Vec<u8> = Vec::new();
        let flow = execute(&line, &mut self.state, &mut buf);
        self.output.push_str(&String::from_utf8_lossy(&buf));

        if let Flow::Exit = flow {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn shell_ui(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&self.output).monospace());

                    ui.horizontal(|ui| {
                        let prompt = build_prompt();
                        ui.label(
                            egui::RichText::new(&prompt)
                                .monospace()
                                .color(egui::Color32::from_rgb(120, 200, 120)),
                        );

                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.input)
                                .font(egui::TextStyle::Monospace)
                                .frame(egui::Frame::NONE)
                                .desired_width(f32::INFINITY),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.run_input(&prompt, &ctx);
                        }
                        response.request_focus();
                    });
                });
        });
    }

    fn fm_ui(&mut self, ui: &mut egui::Ui) {
        let events = ui.input(|i| i.events.clone());
        let mut close = false;
        if let Some(editor) = self.fm.as_mut() {
            'events: for event in &events {
                for (code, modifiers) in keys_from_event(event) {
                    if editor.handle_key(code, modifiers, self.fm_rows) {
                        close = true;
                        break 'events;
                    }
                }
            }
            editor.scroll_to_cursor(self.fm_rows);
        }

        if close {
            if let Some(editor) = self.fm.take() {
                if editor.file_deleted {
                    self.output
                        .push_str(&format!("fm: {} deleted\n", editor.file_path));
                }
            }
            ui.ctx().request_repaint();
            return;
        }

        let Some(editor) = self.fm.as_ref() else {
            return;
        };

        egui::Panel::top("fm_top").show(ui, |ui| {
            ui.horizontal(|ui| {
                let title = format!(
                    " fm {}{}",
                    editor.file_path,
                    if editor.modified { " [modified]" } else { "" }
                );
                ui.label(egui::RichText::new(title).monospace().strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.monospace(format!(
                        "Line {}, Col {}",
                        editor.cursor_row + 1,
                        editor.cursor_col + 1
                    ));
                });
            });
        });

        egui::Panel::bottom("fm_bottom").show(ui, |ui| {
            if editor.status_message.is_empty() {
                ui.horizontal_wrapped(|ui| {
                    for (key, text) in [
                        (" CTRL+X ", " Close  "),
                        (" CTRL+S ", " Save "),
                        (" CTRL+D ", " Delete "),
                        (" CTRL+L ", " Delete line "),
                        (" CTRL+K ", " Cut line "),
                        (" CTRL+U ", " Paste "),
                        (" CTRL+R ", " Rename file "),
                    ] {
                        ui.label(
                            egui::RichText::new(key)
                                .monospace()
                                .color(Color32::BLACK)
                                .background_color(Color32::from_gray(210)),
                        );
                        ui.monospace(text);
                    }
                });
            } else {
                let is_error = editor.status_message.starts_with("Error")
                    || editor.status_message.contains('?');
                let color = if is_error {
                    Color32::from_rgb(139, 0, 0)
                } else {
                    Color32::from_rgb(0, 100, 0)
                };
                egui::Frame::new().fill(color).show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(
                        egui::RichText::new(format!(" {}", editor.status_message))
                            .monospace()
                            .color(Color32::WHITE),
                    );
                });
            }
        });

        let mut rows = self.fm_rows;
        egui::CentralPanel::default().show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let font = egui::TextStyle::Monospace.resolve(ui.style());
            let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
            rows = ((ui.available_height() / row_height).floor() as usize).max(1);

            let number_width = editor.lines.len().to_string().len();
            let text_color = ui.visuals().text_color();
            let background = ui.visuals().panel_fill;

            for (index, line) in editor
                .lines
                .iter()
                .enumerate()
                .skip(editor.scroll_row)
                .take(rows)
            {
                let plain = egui::TextFormat {
                    font_id: font.clone(),
                    color: text_color,
                    ..Default::default()
                };
                let mut job = egui::text::LayoutJob::default();
                job.append(
                    &format!("{:>width$} | ", index + 1, width = number_width),
                    0.0,
                    egui::TextFormat {
                        font_id: font.clone(),
                        color: Color32::DARK_GRAY,
                        ..Default::default()
                    },
                );

                if index == editor.cursor_row {
                    let col = editor.cursor_col.min(line.len());
                    let before: String = line[..col].iter().collect();
                    let at: String = line.get(col).map(|c| c.to_string()).unwrap_or(" ".into());
                    let after: String = line.iter().skip(col + 1).collect();
                    job.append(&before, 0.0, plain.clone());
                    job.append(
                        &at,
                        0.0,
                        egui::TextFormat {
                            font_id: font.clone(),
                            color: background,
                            background: text_color,
                            ..Default::default()
                        },
                    );
                    job.append(&after, 0.0, plain);
                } else {
                    let text: String = line.iter().collect();
                    job.append(&text, 0.0, plain);
                }

                ui.add(egui::Label::new(job).wrap_mode(egui::TextWrapMode::Extend));
            }
        });
        if rows != self.fm_rows {
            self.fm_rows = rows;
            ui.ctx().request_repaint();
        }
    }
}

impl eframe::App for ShellApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.fm.is_some() {
            self.fm_ui(ui);
        } else {
            self.shell_ui(ui);
        }
    }
}

fn main() -> eframe::Result {
    let icon = eframe::icon_data::from_png_bytes(include_bytes!("../../assets/icon.png"))
        .expect("icon.png ist keine gueltige PNG");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 550.0])
            .with_title("Rust Shell")
            .with_app_id("rust-shell")
            .with_icon(icon),
        ..Default::default()
    };
    eframe::run_native(
        "Rust Shell",
        options,
        Box::new(|_cc| {
            Ok(Box::new(ShellApp {
                state: ShellState::new(),
                input: String::new(),
                output: String::new(),
                fm: None,
                fm_rows: 20,
            }))
        }),
    )
}
