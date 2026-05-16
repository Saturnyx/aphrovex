use egui::Vec2;

pub struct AboutWindowState {
    pub open:         bool,
    pub license_open: bool,
}

impl Default for AboutWindowState {
    fn default() -> Self {
        Self {
            open:         false,
            license_open: false,
        }
    }
}

impl AboutWindowState {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("About")
            .open(&mut self.open)
            .fixed_size(Vec2::new(200.0, 200.0))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::default())
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                let title = egui::Label::new(egui::RichText::new("Archipelago").size(18.0))
                    .halign(egui::Align::Center);
                let subtitle = egui::Label::new(env!("CARGO_PKG_DESCRIPTION"));
                let author = egui::Label::new(format!("By {}", env!("CARGO_PKG_AUTHORS")));
                let version = egui::Label::new(format!("Version {}", env!("CARGO_PKG_VERSION")));
                let spacer = egui::Separator::default();

                ui.add(title);
                ui.add(subtitle);
                ui.add(version);
                ui.add(author);
                ui.add(spacer);

                if ui
                    .button(format!("License ({})", env!("CARGO_PKG_LICENSE")))
                    .clicked()
                {
                    self.license_open = !self.license_open;
                }
            });
        if self.license_open {
            egui::Window::new(env!("CARGO_PKG_LICENSE"))
                .open(&mut self.license_open)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let license = egui::Label::new(include_str!("../../LICENSE"));
                        ui.add(license);
                    })
                });
        }
    }
}
