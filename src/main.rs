#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod dict;
use dict::Word;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        ..Default::default()
    };
    eframe::run_native(
        "Türkçe Sözlük",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

struct MyApp {
    word: Word,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { word: Word::new() }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Türkçe Sözlük").;
            ui.horizontal(|ui| {
                let name_label = ui.label("Kelime: ");
                ui.text_edit_singleline(&mut self.word.name)
                    .labelled_by(name_label.id);
                if ui.button("Ara").clicked() {
                    self.word.clear();
                    self.word.get_def().unwrap_or(());
                }
            });

            if self.word.def_count != 0 {
                for (i, def) in self.word.definitions.iter().enumerate() {
                    ui.label(format!("{}. {}", i + 1, def));
                }
            }
        });
    }
}
