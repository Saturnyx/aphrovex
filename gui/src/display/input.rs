use eframe::egui;
use roboscope_ipc::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH, DisplayInputKind};

use super::{DisplayPanel, TouchState};

impl DisplayPanel {
    pub fn handle_input(&mut self, ui: &egui::Ui, display_rect: egui::Rect) {
        let pressed = ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Primary));
        let released = ui.input(|i| i.pointer.button_released(egui::PointerButton::Primary));
        let pointer_down = ui.input(|i| i.pointer.primary_down());
        let pointer_pos = ui.input(|i| i.pointer.latest_pos());

        if pressed {
            if let Some(pos) = pointer_pos {
                if display_rect.contains(pos) {
                    let coords = self.to_display_coords(pos, display_rect);
                    self.touch.is_mouse_down = true;
                    self.touch.num_clicks = self.touch.num_clicks.wrapping_add(1);
                    self.touch.mouse_coords = coords;
                    self.send_input(DisplayInputKind::Press, coords);
                }
            }
        } else if released && self.touch.is_mouse_down {
            let coords = pointer_pos
                .map(|pos| self.to_display_coords(pos, display_rect))
                .unwrap_or(self.touch.mouse_coords);

            self.touch.is_mouse_down = false;
            self.touch.mouse_coords = coords;
            self.send_input(DisplayInputKind::Release, coords);
        } else if pointer_down && self.touch.is_mouse_down {
            if let Some(pos) = pointer_pos {
                let coords = self.to_display_coords(pos, display_rect);
                if coords != self.touch.mouse_coords {
                    self.touch.mouse_coords = coords;
                    self.send_input(DisplayInputKind::Hold, coords);
                }
            }
        }
    }

    /// Pack the current touch state and ship it over the channel to
    /// `ThreadDisplay`, which will relay it to IPC.
    fn send_input(&mut self, _kind: DisplayInputKind, coords: [i16; 2]) {
        let _ = self.touch_tx.send(TouchState {
            is_mouse_down: self.touch.is_mouse_down,
            num_clicks:    self.touch.num_clicks,
            mouse_coords:  coords,
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
