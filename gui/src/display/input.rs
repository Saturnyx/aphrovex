use eframe::egui;
use roboscope_ipc::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, DisplayInput, DisplayInputKind};

use super::DisplayPanel;

impl DisplayPanel {
    pub fn handle_input(&mut self, ui: &egui::Ui, display_rect: egui::Rect) {
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
            self.send_input(DisplayInputKind::Release, self.num_clicks, release_count, coords);
        } else if pointer_down && self.is_mouse_down {
            if let Some(pos) = pointer_pos {
                let coords = self.to_display_coords(pos, display_rect);
                if coords != self.mouse_coords {
                    self.mouse_coords = coords;
                    let release_count = self.num_clicks.wrapping_sub(1);
                    self.send_input(DisplayInputKind::Hold, self.num_clicks, release_count, coords);
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
