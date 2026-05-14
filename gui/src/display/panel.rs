use eframe::egui;

use roboscope_ipc::display::{DISPLAY_HEIGHT, DISPLAY_UPDATE_PERIOD, DISPLAY_WIDTH};

use super::DisplayPanel;

impl DisplayPanel {
    pub fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        if !*open {
            return;
        }

        self.ensure_ipc();

        ctx.request_repaint_after(*DISPLAY_UPDATE_PERIOD);

        egui::Window::new("Brain Display")
            .open(open)
            .resizable(true)
            .collapsible(false)
            .default_size(egui::vec2(DISPLAY_WIDTH as f32, DISPLAY_HEIGHT as f32))
            .min_size(egui::vec2(DISPLAY_WIDTH as f32, DISPLAY_HEIGHT as f32))
            .show(ctx, |ui| {
                ui.set_min_size(egui::vec2(DISPLAY_WIDTH as f32, DISPLAY_HEIGHT as f32));

                if let Some(err) = &self.init_error {
                    ui.label(err);
                    return;
                }

                let updated = self.poll_frame();
                if updated || self.texture.is_none() {
                    self.update_texture(ui.ctx());
                }

                let available_rect = ui.available_rect_before_wrap();
                let display_size = egui::vec2(DISPLAY_WIDTH as f32, DISPLAY_HEIGHT as f32);
                let scale = (available_rect.width() / display_size.x)
                    .min(available_rect.height() / display_size.y);

                let scale = if scale.is_finite() && scale > 0.0 {
                    scale
                } else {
                    1.0
                };
                let scaled_size = display_size * scale;
                let display_rect =
                    egui::Rect::from_center_size(available_rect.center(), scaled_size);

                let _response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());

                let painter = ui.painter();
                painter.rect_filled(available_rect, 0.0, ui.visuals().extreme_bg_color);

                if let Some(texture) = &self.texture {
                    let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
                    painter.image(texture.id(), display_rect, uv, egui::Color32::WHITE);
                } else {
                    painter.text(
                        display_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Waiting for frame...",
                        egui::TextStyle::Body.resolve(ui.style()),
                        ui.visuals().text_color(),
                    );
                }

                self.handle_input(ui, display_rect);
            });
    }
}
