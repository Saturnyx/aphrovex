use bytemuck::cast_slice;
use egui::{ColorImage, TextureOptions};

use roboscope_ipc::display::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

use super::DisplayPanel;

impl DisplayPanel {
    pub fn update_texture(&mut self, ctx: &egui::Context) {
        let Some(frame) = &self.last_frame else {
            return;
        };

        let size = [DISPLAY_WIDTH as usize, DISPLAY_HEIGHT as usize];
        let mut rgba = cast_slice(&frame.buffer).to_vec();

        // The display buffer is packed as BGRx/ARGB in native endianness.
        // Convert to RGBA and force opaque alpha so egui shows it correctly.
        for pixel in rgba.chunks_exact_mut(4) {
            pixel.swap(0, 2); // swap R/B
            pixel[3] = 0xFF;
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
}
