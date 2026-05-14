use bytemuck::cast_slice;
use eframe::egui;
use egui::{ColorImage, TextureHandle, TextureOptions};

use iceoryx2::{node::NodeBuilder, signal_handling_mode::SignalHandlingMode};

use roboscope_ipc::{
    Config, Publisher, SimServices, Subscriber,
    display::{
        DISPLAY_HEIGHT, DISPLAY_UPDATE_PERIOD, DISPLAY_WIDTH, DisplayFrame, DisplayInput,
        DisplayInputKind,
    },
};

pub struct DisplayPanel {
    ipc: Option<SimServices>,
    subscriber: Option<Subscriber<DisplayFrame>>,
    publisher: Option<Publisher<DisplayInput>>,
    last_frame: Option<Box<DisplayFrame>>,
    texture: Option<TextureHandle>,
    init_error: Option<String>,
    is_mouse_down: bool,
    num_clicks: u32,
    mouse_coords: [i16; 2],
}

impl Default for DisplayPanel {
    fn default() -> Self {
        Self {
            ipc: None,
            subscriber: None,
            publisher: None,
            last_frame: None,
            texture: None,
            init_error: None,
            is_mouse_down: false,
            num_clicks: 0,
            mouse_coords: [0, 0],
        }
    }
}

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

    fn ensure_ipc(&mut self) {
        if self.subscriber.is_some() || self.init_error.is_some() {
            return;
        }

        if let Err(err) = self.try_init_ipc() {
            self.init_error = Some(err);
        }
    }

    fn try_init_ipc(&mut self) -> Result<(), String> {
        let builder = NodeBuilder::new()
            .config(&Config::default())
            .signal_handling_mode(SignalHandlingMode::Disabled);

        let ipc = SimServices::custom(Some("viewer"), builder).map_err(|e| e.to_string())?;

        let subscriber = ipc
            .display_frames()
            .map_err(|e| e.to_string())?
            .subscriber_builder()
            .create()
            .map_err(|e| e.to_string())?;

        let publisher = ipc
            .display_input()
            .map_err(|e| e.to_string())?
            .publisher_builder()
            .create()
            .map_err(|e| e.to_string())?;

        self.ipc = Some(ipc);
        self.subscriber = Some(subscriber);
        self.publisher = Some(publisher);

        Ok(())
    }

    fn poll_frame(&mut self) -> bool {
        let Some(subscriber) = &self.subscriber else {
            return false;
        };

        match subscriber.receive() {
            Ok(Some(frame)) => {
                self.last_frame = Some(Box::new(frame.clone()));
                true
            }
            Ok(None) => false,
            Err(_) => false,
        }
    }

    fn update_texture(&mut self, ctx: &egui::Context) {
        let Some(frame) = &self.last_frame else {
            return;
        };

        let size = [DISPLAY_WIDTH as usize, DISPLAY_HEIGHT as usize];
        let mut rgba = cast_slice(&frame.buffer).to_vec();

        // The display buffer uses a packed u32 format without meaningful alpha,
        // which ends up as fully transparent in egui. Force opaque alpha so
        // the image is visible.
        for alpha in rgba.iter_mut().skip(3).step_by(4) {
            *alpha = 0xFF;
        }

        let image = ColorImage::from_rgba_unmultiplied(size, &rgba);

        match &mut self.texture {
            Some(texture) => texture.set(image, TextureOptions::NEAREST),
            None => {
                self.texture =
                    Some(ctx.load_texture("vex_display", image, TextureOptions::NEAREST));
            }
        }
    }

    fn handle_input(&mut self, ui: &egui::Ui, display_rect: egui::Rect) {
        if self.publisher.is_none() {
            return;
        }

        let pressed = ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary));
        let released = ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary));
        let pointer_down = ui.input(|i| i.pointer.primary_down());
        let pointer_pos = ui.input(|i| i.pointer.latest_pos());

        if pressed {
            if let Some(pos) = pointer_pos {
                if display_rect.contains(pos) {
                    let coords = self.to_display_coords(pos, display_rect);
                    let release_count = self.num_clicks;
                    self.is_mouse_down = true;
                    self.num_clicks = self.num_clicks.wrapping_add(1);
                    self.mouse_coords = coords;
                    self.send_input(
                        DisplayInputKind::Press,
                        self.num_clicks,
                        release_count,
                        coords,
                    );
                }
            }
        } else if released && self.is_mouse_down {
            let coords = pointer_pos
                .map(|pos| self.to_display_coords(pos, display_rect))
                .unwrap_or(self.mouse_coords);

            self.is_mouse_down = false;
            let release_count = self.num_clicks;
            self.mouse_coords = coords;
            self.send_input(
                DisplayInputKind::Release,
                self.num_clicks,
                release_count,
                coords,
            );
        } else if pointer_down && self.is_mouse_down {
            if let Some(pos) = pointer_pos {
                let coords = self.to_display_coords(pos, display_rect);
                if coords != self.mouse_coords {
                    self.mouse_coords = coords;
                    let release_count = self.num_clicks.wrapping_sub(1);
                    self.send_input(
                        DisplayInputKind::Hold,
                        self.num_clicks,
                        release_count,
                        coords,
                    );
                }
            }
        }
    }

    fn send_input(
        &mut self,
        kind: DisplayInputKind,
        press_count: u32,
        release_count: u32,
        coords: [i16; 2],
    ) {
        let Some(publisher) = &self.publisher else {
            return;
        };

        let _ = publisher.send_copy(DisplayInput {
            kind,
            press_count,
            release_count,
            x: coords[0],
            y: coords[1],
        });
    }

    fn to_display_coords(&self, pos: egui::Pos2, display_rect: egui::Rect) -> [i16; 2] {
        let width = display_rect.width().max(1.0);
        let height = display_rect.height().max(1.0);

        let x = ((pos.x - display_rect.min.x) / width) * DISPLAY_WIDTH as f32;
        let y = ((pos.y - display_rect.min.y) / height) * DISPLAY_HEIGHT as f32;

        let x = x.clamp(0.0, (DISPLAY_WIDTH - 1) as f32) as i16;
        let y = y.clamp(0.0, (DISPLAY_HEIGHT - 1) as f32) as i16;

        [x, y]
    }
}
