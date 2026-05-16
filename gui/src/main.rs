#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod about;
mod close_dialog;
mod display;
mod menubar;
use close_dialog::CloseWindowState;
use eframe::egui;

use crate::{about::AboutWindowState, display::DisplayWindowState};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("Confirm exit", options, Box::new(|_cc| Ok(Box::<App>::default())))
}
struct App {
    pub close_dialog: CloseWindowState,
    pub display:      DisplayWindowState,
    pub about:        AboutWindowState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            close_dialog: CloseWindowState::default(),
            display:      DisplayWindowState::default(),
            about:        AboutWindowState::default(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menubar(ui);
        self.display
            .display_panel
            .show(ui.ctx(), &mut self.display.display_open);
        self.close_dialog.update(ui);
        self.about.show(ui.ctx());
    }
}
