#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod about;
mod close_dialog;
mod display;
mod distance;
mod ipc;
mod menubar;
mod motors;
mod prefs;

use std::sync::Arc;

use close_dialog::CloseWindowState;
use eframe::egui;
use egui::Color32;

use crate::{
    about::AboutWindowState,
    display::DisplayWindowState,
    distance::DistanceWindowState,
    ipc::AppIpc,
    motors::MotorWindowState,
    prefs::{PrefWindow, Preferences},
};

const APP_NAME: &str = "Archipelago";

fn main() -> eframe::Result {
    env_logger::init();
    let icon_bytes = include_bytes!("../assets/img/export/starfish_32.png");
    let icon = image::load_from_memory(icon_bytes).expect("Whoops, couldn't find icon");
    let icon_data = egui::IconData {
        rgba:   icon
            .as_rgba8()
            .expect("Unable to load icon")
            .as_raw()
            .clone(),
        width:  icon.width(),
        height: icon.height(),
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(APP_NAME)
            .with_taskbar(true)
            .with_transparent(true)
            .with_icon(Arc::new(icon_data))
            .with_maximized(true),
        ..Default::default()
    };
    let (app_ipc, mut thread_ipc) = AppIpc::new();
    std::thread::spawn(move || {
        thread_ipc.thread_update();
    });
    eframe::run_native(
        env!("CARGO_BIN_NAME"),
        options,
        Box::new(|cc| {
            let prefs = cc
                .storage
                .and_then(|storage| eframe::get_value(storage, eframe::APP_KEY))
                .unwrap_or_default();
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let app = App::from_ipc(app_ipc, prefs);
            Ok(Box::new(app))
        }),
    )
}

struct App {
    pub close_dialog: CloseWindowState,
    pub display:      DisplayWindowState,
    pub about:        AboutWindowState,
    pub motors:       Vec<MotorWindowState>,
    pub distance:     Vec<DistanceWindowState>,
    pub prefs:        PrefWindow,
    pub ipc:          AppIpc,
}

impl App {
    fn from_ipc(ipc: AppIpc, prefs: Preferences) -> Self {
        Self {
            close_dialog: CloseWindowState::default(),
            display: DisplayWindowState::default(),
            about: AboutWindowState::default(),
            motors: Vec::new(),
            distance: Vec::new(),
            prefs: PrefWindow::from_prefs(prefs),
            ipc,
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(self.prefs.prefs.theme.to_visuals());
        self.menubar(ui);
        self.prefs.show(ui.ctx());
        self.display.show(ui.ctx(), &mut self.ipc);
        self.close_dialog.update(ui, &self.prefs.prefs);
        self.about.show(ui.ctx());
        for motor in &mut self.motors {
            motor.show(ui.ctx(), &mut self.ipc, &self.prefs.prefs);
        }
        for distance in &mut self.distance {
            distance.show(ui.ctx(), &mut self.ipc, &self.prefs.prefs);
        }
        self.ipc.sync();
        self.motors.retain(|m| m.open);
        self.distance.retain(|m| m.open);
        let frame = egui::Frame::default().fill(Color32::from_black_alpha(
            (self.prefs.prefs.transparent_window * 255.0) as u8,
        ));

        egui::CentralPanel::default()
            .frame(frame)
            .show_inside(ui, |_| {});
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Direct eframe to serialize your preferences struct using the same APP_KEY
        eframe::set_value(storage, eframe::APP_KEY, &self.prefs.prefs);
    }
}
