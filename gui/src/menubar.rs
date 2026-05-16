use eframe::egui;

use crate::about::AboutState;

pub fn menubar(ui: &mut egui::Ui, display_open: &mut bool, about_state: &mut AboutState) {
    egui::Panel::top("menu_bar").show_inside(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("About").clicked() {
                    about_state.open = true;
                }
                if ui.button("Quit").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.menu_button("View", |ui| {
                ui.checkbox(display_open, "Display");
            });
        });
    });
}
