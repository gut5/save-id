mod core;
mod db;
mod scanner;

use eframe::egui;
use rfd::FileDialog;

use crate::db::GameDB;
use crate::scanner::Scanner;
use crate::core::ScanResult;

struct App {
    folder: Option<String>,
    results: Vec<ScanResult>,
    db_path: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            folder: None,
            results: vec![],
            db_path: "games.db".to_string(),
        }
    }
}

impl App {
    fn run_scan(&mut self) {
        if let Some(folder) = &self.folder {
            let db = GameDB::new(&self.db_path);
            let scanner = Scanner::new(db);

            self.results = scanner.scan(folder);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {        //egui::CentralPanel::default().show_inside(ctx, |ui| {
            ui.heading("Game Save Scanner");

            if ui.button("Select Folder").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.folder = Some(path.display().to_string());
                }
            }

            if let Some(folder) = &self.folder {
                ui.label(format!("Folder: {}", folder));
            }

            if ui.button("Scan").clicked() {
                self.run_scan();
            }

            ui.separator();

            ui.label(format!("Results: {}", self.results.len()));

            egui::ScrollArea::vertical().show(ui, |ui| {
                for r in &self.results {
                    ui.horizontal(|ui| {
                        ui.label(r.path.clone());
                        ui.separator();
                        ui.label(r.code.clone().unwrap_or("-".to_string()));
                        ui.separator();
                        ui.label(r.game_name.clone().unwrap_or("Unknown".to_string()));
                        ui.separator();
                        ui.label(r.console.clone().unwrap_or("-".to_string()));
                    });
                }
            });
        //});
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Game Save Scanner",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}