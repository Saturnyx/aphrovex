use roboscope_ipc::SimServices;

use super::DisplayPanel;
use crate::ipc::AppIpc;

impl DisplayPanel {
    pub fn ensure_ipc(&mut self, ipc: &mut AppIpc) {
        if self.subscriber.is_some() || self.init_error.is_some() {
            return;
        }

        ipc.ensure();
        let Some(simservices) = ipc.ipc() else {
            return;
        };

        if let Err(err) = self.try_init_ipc(simservices) {
            self.init_error = Some(err);
        }
    }

    fn try_init_ipc(&mut self, ipc: &SimServices) -> Result<(), String> {
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
