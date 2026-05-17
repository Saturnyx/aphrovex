use eframe::egui;
use egui::Vec2;

pub struct CloseWindowState {
    open:             bool,
    allowed_to_close: bool,
}

impl CloseWindowState {
    pub fn check_close(&mut self, ui: &mut egui::Ui) {
        if ui.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ui.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.open = true;
            }
        }
    }

    pub fn show_confirmation_dialog(&mut self, ui: &mut egui::Ui) {
        if self.open {
            egui::Window::new("Do you want to quit?")
                .anchor(egui::Align2::CENTER_CENTER, Vec2::default())
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.open = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.open = false;
                            self.allowed_to_close = true;
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        self.check_close(ui);
        self.show_confirmation_dialog(ui);
    }
}

impl Default for CloseWindowState {
    fn default() -> Self {
        Self {
            open:             false,
            allowed_to_close: false,
        }
    }
}
