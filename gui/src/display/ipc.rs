use roboscope_ipc::Config;

use crate::{display::ThreadDisplay, ipc::AppIpc};

impl ThreadDisplay {
    pub fn ensure_ipc(&mut self, _ipc: &mut AppIpc) {
        if self.subscriber.is_some() || self.init_error.is_some() {
            return;
        }
        if let Err(err) = self.try_init_ipc() {
            self.init_error = Some(err);
        }
    }

    fn try_init_ipc(&mut self) -> Result<(), String> {
        let services = roboscope_ipc::SimServices::join(Some("display-viewer"), &Config::default())
            .map_err(|e| e.to_string())?;

        let subscriber = services
            .display_frames()
            .map_err(|e| e.to_string())?
            .subscriber_builder()
            .create()
            .map_err(|e| e.to_string())?;

        let publisher = services
            .display_input()
            .map_err(|e| e.to_string())?
            .publisher_builder()
            .create()
            .map_err(|e| e.to_string())?;

        self.subscriber = Some(subscriber);
        self.publisher = Some(publisher);

        Ok(())
    }

    /// Poll for a new frame from IPC and forward it to the GUI thread via
    /// `frame_tx`. Returns `true` if a frame was sent.
    pub fn poll_and_forward_frame(&mut self) -> bool {
        let Some(subscriber) = &self.subscriber else {
            return false;
        };

        match subscriber.receive() {
            Ok(Some(sample)) => {
                let frame = (*sample).clone();
                let _ = self.frame_tx.send(Box::new(frame));
                true
            }
            Ok(None) => false,
            Err(_) => false,
        }
    }

    /// Forward any pending touch events from the GUI thread to IPC.
    pub fn forward_touch(&mut self) {
        use roboscope_ipc::display::{DisplayInput, DisplayInputKind};

        let Some(publisher) = &self.publisher else {
            while self.touch_rx.try_recv().is_ok() {}
            return;
        };

        while let Ok(touch) = self.touch_rx.try_recv() {
            let _ = publisher.send_copy(DisplayInput {
                kind:          DisplayInputKind::Hold,
                press_count:   touch.num_clicks,
                release_count: touch.num_clicks.wrapping_sub(1),
                x:             touch.mouse_coords[0],
                y:             touch.mouse_coords[1],
            });
        }
    }
}
