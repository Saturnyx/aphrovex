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
}
