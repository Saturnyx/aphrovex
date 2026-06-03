use eframe::egui;
use egui::Modifiers;

use crate::App;

impl App {
    pub fn menubar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.label(env!("CARGO_BIN_NAME"));
                ui.separator();
                ui.menu_button("File", |ui| {
                    if ui
                        .add(egui::Button::new("About").shortcut_text("ctrl+a"))
                        .clicked()
                    {
                        self.about.open = true;
                    }
                    if ui
                        .add(egui::Button::new("Preferences").shortcut_text("ctrl+p"))
                        .clicked()
                    {
                        self.prefs.open = true;
                    }

                    if ui
                        .add(egui::Button::new("Quit").shortcut_text("ctrl+q"))
                        .clicked()
                    {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Add", |ui| {
                    if ui
                        .add(egui::Button::new("Display").shortcut_text("ctrl+d"))
                        .clicked()
                    {
                        self.display.open = !self.display.open;
                    }
                    if ui
                        .add(egui::Button::new("Motor").shortcut_text("ctrl+m"))
                        .clicked()
                    {
                        self.motors.push(Default::default());
                    }
                    if ui
                        .add(egui::Button::new("Distance Sensor").shortcut_text("ctrl+s"))
                        .clicked()
                    {
                        self.distance.push(Default::default());
                    }
                });
            });
        });
    }

    pub fn shortcuts(&mut self, ui: &mut egui::Ui) {
        let about_key = egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::A);
        let display_key = egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::D);
        let motor_key = egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::M);
        let distance_key = egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::S);
        let prefs_key = egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::P);
        if ui.input_mut(|i| i.consume_shortcut(&about_key)) {
            self.about.open = !self.about.open;
        }
        if ui.input_mut(|i| i.consume_shortcut(&display_key)) {
            self.display.open = !self.display.open;
        }
        if ui.input_mut(|i| i.consume_shortcut(&motor_key)) {
            self.motors.push(Default::default());
        }
        if ui.input_mut(|i| i.consume_shortcut(&distance_key)) {
            self.distance.push(Default::default());
        }
        if ui.input_mut(|i| i.consume_shortcut(&prefs_key)) {
            self.prefs.open = !self.prefs.open;
        }
    }
}
