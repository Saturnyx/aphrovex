#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod about;
mod close_dialog;
mod display;
mod ipc;
mod menubar;
mod motors;
mod prefs;

use close_dialog::CloseWindowState;
use eframe::egui;

use crate::{
    about::AboutWindowState,
    display::DisplayWindowState,
    ipc::AppIpc,
    motors::MotorWindowState,
    prefs::PrefWindow,
};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    let (app_ipc, mut thread_ipc) = AppIpc::new();
    std::thread::spawn(move || {
        thread_ipc.thread_update();
    });
    eframe::run_native(
        env!("CARGO_BIN_NAME"),
        options,
        Box::new(|_cc| {
            let app = App::from_ipc(app_ipc);
            Ok(Box::new(app))
        }),
    )
}

struct App {
    pub close_dialog: CloseWindowState,
    pub display:      DisplayWindowState,
    pub about:        AboutWindowState,
    pub motors:       Vec<MotorWindowState>,
    pub prefs:        PrefWindow,
    pub ipc:          AppIpc,
}

impl App {
    fn from_ipc(ipc: AppIpc) -> Self {
        Self {
            close_dialog: CloseWindowState::default(),
            display: DisplayWindowState::default(),
            about: AboutWindowState::default(),
            motors: Vec::new(),
            prefs: PrefWindow::default(),
            ipc,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.menubar(ui);
        self.display.show(ui.ctx(), &mut self.ipc);
        self.close_dialog.update(ui);
        self.about.show(ui.ctx());
        self.prefs.show(ui.ctx());
        for motor in &mut self.motors {
            motor.show(ui.ctx(), &mut self.ipc, &self.prefs.prefs);
        }
        self.ipc.sync();
        self.motors.retain(|m| m.open);
    }
}
