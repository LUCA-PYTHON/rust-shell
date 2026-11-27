#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use rust_shell::state::ShellState;
use rust_shell::{Flow, execute};

struct ShellApp {
    state: ShellState,
    input: String,
    output: String,
}

fn build_promt() -> String {
    match std::env::current_dir() {
        Ok(path) => format!("rust-shell: {}>", path.display()),
        Err(..) => String::from("rust-shell>"),
    }
}

impl eframe::App for ShellApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        // Eingabezeile unten
        egui::Panel::bottom("input").show(ui, |ui| {
            ui.horizontal(|ui| {
                let prompt = build_promt();
                ui.monospace(&prompt);
                let response = ui.add(
                    egui::TextEdit::singleline(&mut self.input)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );

                // Enter gedrueckt: Zeile ausfuehren
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let line = std::mem::take(&mut self.input);
                    self.output.push_str(&format!("{} {}\n", prompt, line));

                    let mut buf: Vec<u8> = Vec::new();
                    let flow = execute(&line, &mut self.state, &mut buf);
                    self.output.push_str(&String::from_utf8_lossy(&buf));

                    if let Flow::Exit = flow {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                response.request_focus();
            });
        });

        // Ausgabe darueber
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical()
                .stick_to_bottom(true)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(&self.output).monospace());
                });
        });
    }
}

fn main() -> eframe::Result {
    // Fenster-Icon (Taskleiste / Titelleiste), die PNG wird in die Programmdatei eingebettet
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
            }))
        }),
    )
}