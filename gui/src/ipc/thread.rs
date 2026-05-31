use std::{sync::mpsc, thread::sleep, time::Duration};

use iceoryx2::{node::NodeBuilder, signal_handling_mode::SignalHandlingMode};
use roboscope_ipc::{Config, SimServices, cmd::RobotOutputs, snapshot::DeviceReadings};

pub struct ThreadIpc {
    pub ipc: Option<SimServices>,

    pub readings_rx: mpsc::Receiver<DeviceReadings>,
    pub output_tx:   mpsc::Sender<RobotOutputs>,
    pub error_tx:    mpsc::Sender<String>,
}

impl ThreadIpc {
    fn try_init(&mut self) -> Result<(), String> {
        let builder = NodeBuilder::new()
            .config(&Config::default())
            .signal_handling_mode(SignalHandlingMode::Disabled);

        let ipc = SimServices::custom(Some("viewer"), builder).map_err(|e| e.to_string())?;

        self.ipc = Some(ipc);

        Ok(())
    }

    /// Blocking loop — run this on a dedicated thread via `std::thread::spawn`.
    pub fn thread_update(&mut self) {
        if let Err(e) = self.try_init() {
            let _ = self.error_tx.send(e);
            return;
        }
        let Some(ipc) = &self.ipc else { return };

        let cmd_subscriber = match ipc.device_cmds() {
            Ok(s) => match s.subscriber_builder().create() {
                Ok(s) => s,
                Err(e) => {
                    let _ = self.error_tx.send(e.to_string());
                    return;
                }
            },
            Err(e) => {
                let _ = self.error_tx.send(e.to_string());
                return;
            }
        };

        let readings_publisher = match ipc.device_readings() {
            Ok(p) => match p.publisher_builder().create() {
                Ok(p) => p,
                Err(e) => {
                    let _ = self.error_tx.send(e.to_string());
                    return;
                }
            },
            Err(e) => {
                let _ = self.error_tx.send(e.to_string());
                return;
            }
        };

        let mut readings = DeviceReadings::default(); // ← outside the loop
        let mut output = RobotOutputs::default(); // ← outside the loop

        loop {
            while let Ok(r) = self.readings_rx.try_recv() {
                readings = r;
            }

            if let Ok(sample) = readings_publisher.loan_uninit() {
                let _ = sample.write_payload(readings.clone()).send();
            }

            while let Ok(Some(sample)) = cmd_subscriber.receive() {
                output = (*sample).clone();
            }
            let _ = self.output_tx.send(output.clone());

            sleep(Duration::from_millis(10));
        }
    }
}
