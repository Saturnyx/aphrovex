use iceoryx2::{node::NodeBuilder, signal_handling_mode::SignalHandlingMode};
use roboscope_ipc::{Config, SimServices};

use super::DisplayPanel;

impl DisplayPanel {
    pub fn ensure_ipc(&mut self) {
        if self.subscriber.is_some() || self.init_error.is_some() {
            return;
        }

        if let Err(err) = self.try_init_ipc() {
            self.init_error = Some(err);
        }
    }

    pub fn try_init_ipc(&mut self) -> Result<(), String> {
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

    pub fn poll_frame(&mut self) -> bool {
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
}
