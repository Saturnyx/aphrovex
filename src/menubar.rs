use eframe::egui;

use crate::App;

impl App {
    pub fn menubar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.label(env!("CARGO_BIN_NAME"));
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui.button("About").clicked() {
                        self.about.open = true;
                    }
                    if ui.button("Preferences").clicked() {
                        self.prefs.open = true;
                    }

                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Add", |ui| {
                    ui.checkbox(&mut self.display.open, "Display");
                    if ui.button("Motor").clicked() {
                        self.motors.push(Default::default());
                    }
                    if ui.button("Distance Sensor").clicked() {
                        self.distance.push(Default::default());
                    }
                });
            });
        });
    }
}
