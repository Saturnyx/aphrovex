#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod about;
mod close_dialog;
mod display;
mod menubar;
use close_dialog::CloseState;
use display::DisplayPanel;
use eframe::egui;

use crate::about::AboutState;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("Confirm exit", options, Box::new(|_cc| Ok(Box::<App>::default())))
}
struct App {
    pub close_dialog:  CloseState,
    pub display_panel: DisplayPanel,
    pub display_open:  bool,
    pub about_panel:   AboutState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            close_dialog:  CloseState::default(),
            display_panel: DisplayPanel::default(),
            display_open:  true,
            about_panel:   AboutState::default(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menubar(ui);
        self.display_panel.show(ui.ctx(), &mut self.display_open);
        self.close_dialog.update(ui);
        self.about_panel.show(ui.ctx());
    }
}
