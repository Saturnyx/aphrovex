use eframe::egui;
use egui::Vec2;

pub struct CloseState {
    show_confirmation_dialog: bool,
    allowed_to_close:         bool,
}

impl CloseState {
    pub fn check_close(&mut self, ui: &mut egui::Ui) {
        if ui.input(|i| i.viewport().close_requested()) {
            if self.allowed_to_close {
                // do nothing - we will close
            } else {
                ui.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }
    }

    pub fn show_confirmation_dialog(&mut self, ui: &mut egui::Ui) {
        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .anchor(egui::Align2::CENTER_CENTER, Vec2::default())
                .collapsible(false)
                .resizable(false)
                .show(ui.ctx(), |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
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

impl Default for CloseState {
    fn default() -> Self {
        Self {
            show_confirmation_dialog: false,
            allowed_to_close:         false,
        }
    }
}
