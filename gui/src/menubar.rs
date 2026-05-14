use eframe::egui;

pub fn menubar(ui: &mut egui::Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    });
}
