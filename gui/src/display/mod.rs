mod input;
mod ipc;
mod texture;
mod window;

use egui::TextureHandle;
use roboscope_ipc::{
    Publisher,
    Subscriber,
    display::{DisplayFrame, DisplayInput},
};

pub struct DisplayWindowState {
    pub open:          bool,
    pub display_panel: DisplayPanel,
}

impl Default for DisplayWindowState {
    fn default() -> Self {
        Self {
            open:          true,
            display_panel: DisplayPanel::default(),
        }
    }
}

pub struct DisplayPanel {
    subscriber:    Option<Subscriber<DisplayFrame>>,
    publisher:     Option<Publisher<DisplayInput>>,
    last_frame:    Option<Box<DisplayFrame>>,
    texture:       Option<TextureHandle>,
    init_error:    Option<String>,
    is_mouse_down: bool,
    num_clicks:    u32,
    mouse_coords:  [i16; 2],
}

impl Default for DisplayPanel {
    fn default() -> Self {
        Self {
            subscriber:    None,
            publisher:     None,
            last_frame:    None,
            texture:       None,
            init_error:    None,
            is_mouse_down: false,
            num_clicks:    0,
            mouse_coords:  [0, 0],
        }
    }
}
