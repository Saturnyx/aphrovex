mod input;
mod ipc;
mod texture;
mod window;
use std::sync::mpsc::*;

use egui::TextureHandle;
use roboscope_ipc::{
    Publisher,
    Subscriber,
    display::{DisplayFrame, DisplayInput},
};

pub struct DisplayWindowState {
    pub open:          bool,
    pub display_panel: DisplayPanel,
    pub thread:        ThreadDisplay,
}

impl Default for DisplayWindowState {
    fn default() -> Self {
        let (display_panel, thread) = DisplayPanel::new();
        Self {
            open: true,
            display_panel,
            thread,
        }
    }
}

pub struct TouchState {
    is_mouse_down: bool,
    num_clicks:    u32,
    mouse_coords:  [i16; 2],
}

pub struct DisplayPanel {
    texture:  Option<TextureHandle>,
    touch:    TouchState,
    frame_rx: Receiver<Box<DisplayFrame>>,
    touch_tx: Sender<TouchState>,
}

pub struct ThreadDisplay {
    subscriber: Option<Subscriber<DisplayFrame>>,
    publisher:  Option<Publisher<DisplayInput>>,
    init_error: Option<String>,
    frame_tx:   Sender<Box<DisplayFrame>>,
    touch_rx:   Receiver<TouchState>,
}

impl DisplayPanel {
    fn new() -> (Self, ThreadDisplay) {
        let (frame_tx, frame_rx) = channel();
        let (touch_tx, touch_rx) = channel();

        let display = Self {
            texture: None,
            touch: TouchState {
                is_mouse_down: false,
                num_clicks:    0,
                mouse_coords:  [0, 0],
            },
            frame_rx,
            touch_tx,
        };

        let thread = ThreadDisplay {
            subscriber: None,
            publisher: None,
            init_error: None,
            frame_tx,
            touch_rx,
        };

        (display, thread)
    }

    /// Drain all pending frames from the background thread, returning true
    /// if at least one new frame arrived (i.e. the texture needs a refresh).
    pub fn recv_frame(&mut self) -> Option<Box<DisplayFrame>> {
        let mut latest = None;
        while let Ok(frame) = self.frame_rx.try_recv() {
            latest = Some(frame);
        }
        latest
    }
}
