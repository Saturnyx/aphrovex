use std::sync::mpsc;

use roboscope_ipc::{cmd::RobotOutputs, snapshot::DeviceReadings};

mod thread;

use thread::ThreadIpc;

pub struct AppIpc {
    pub readings:   DeviceReadings,
    pub output:     RobotOutputs,
    pub init_error: Option<String>,

    readings_tx: mpsc::Sender<DeviceReadings>,
    output_rx:   mpsc::Receiver<RobotOutputs>,
    error_rx:    mpsc::Receiver<String>,
}

impl AppIpc {
    pub fn new() -> (Self, ThreadIpc) {
        let (readings_tx, readings_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let (error_tx, error_rx) = mpsc::channel();

        let gui = Self {
            readings: DeviceReadings::default(),
            output: RobotOutputs::default(),
            init_error: None,
            readings_tx,
            output_rx,
            error_rx,
        };

        let bg = ThreadIpc {
            ipc: None,
            readings_rx,
            output_tx,
            error_tx,
        };

        (gui, bg)
    }

    /// Call once per egui frame. Pushes readings to the background thread,
    /// pulls output and errors back. Never blocks.
    pub fn sync(&mut self) {
        // Push latest readings (fire-and-forget; background drains and keeps last)
        let _ = self.readings_tx.send(self.readings.clone());

        // Drain output updates — last write wins
        while let Ok(output) = self.output_rx.try_recv() {
            self.output = output;
        }

        // Drain any init errors sent by the background thread
        while let Ok(err) = self.error_rx.try_recv() {
            self.init_error = Some(err);
        }
    }

    pub fn init_error(&self) -> Option<&str> { self.init_error.as_deref() }
}
