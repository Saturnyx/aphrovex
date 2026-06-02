use rand::Rng;
use roboscope_ipc::snapshot::DistanceSnapshot;

use crate::{ipc::AppIpc, prefs::Preferences};

const NO_DISTANCE_READING_VALUE: u32 = 9999;
const NO_CONFIDENCE_READING_VALUE: u32 = 10;

pub struct DistanceWindowState {
    pub id:                 u64,
    pub open:               bool,
    pub port:               usize,
    pub distance_enabled:   bool,
    pub confidence_enabled: bool,
    pub initializing:       bool,
    pub snapshot:           DistanceSnapshot,
}

impl Default for DistanceWindowState {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            id:                 rng.next_u64(),
            open:               true,
            port:               0,
            distance_enabled:   true,
            confidence_enabled: true,
            snapshot:           DistanceSnapshot::default(),
            initializing:       false,
        }
    }
}

impl DistanceWindowState {
    pub fn show(&mut self, ctx: &egui::Context, ipc: &mut AppIpc, prefs: &Preferences) {
        let mut port_num: i32 = self.port as i32 + 1; // slider_value: 1..=21

        egui::Window::new("Distance Sensor")
            .collapsible(true)
            .open(&mut self.open)
            .id(egui::Id::new(self.id))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Port (1-21)");
                    if ui.add(egui::Slider::new(&mut port_num, 1..=21)).changed() {
                        self.port = (port_num - 1) as usize; // self.port: 0..=20
                    }
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.distance_enabled, "Distance (in mm)");

                    ui.add_enabled_ui(self.distance_enabled, |ui| {
                        let mut dist_val = self.snapshot.distance as i32;
                        if ui
                            .add(egui::Slider::new(&mut dist_val, 20..=2000))
                            .changed()
                        {
                            self.snapshot.distance = dist_val as _;
                        }
                    });

                    if !self.distance_enabled {
                        self.snapshot.distance = NO_DISTANCE_READING_VALUE;
                    } else {
                        if self.snapshot.distance == NO_DISTANCE_READING_VALUE {
                            self.snapshot.distance = 2000;
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.confidence_enabled, "Confidence");

                    ui.add_enabled_ui(self.confidence_enabled, |ui| {
                        let mut confidence_val = self.snapshot.confidence as i32;
                        if ui
                            .add(egui::Slider::new(&mut confidence_val, 0..=63))
                            .changed()
                        {
                            self.snapshot.confidence = confidence_val as _;
                        }
                    });

                    if !self.confidence_enabled {
                        self.snapshot.confidence = NO_CONFIDENCE_READING_VALUE;
                    } else {
                        if self.snapshot.confidence == NO_CONFIDENCE_READING_VALUE {
                            self.snapshot.confidence = 63;
                        }
                    }
                });
                ui.add(
                    egui::Slider::new(&mut self.snapshot.object_size, 0..=400).text("Object Size"),
                );
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Object Velocity {}/{}",
                        prefs.get_length_units_as_string(),
                        prefs.get_time_units_as_string()
                    ));
                    let mut display_velocity = self.snapshot.object_velocity as f32 *
                        prefs.get_length_units_in_meters() /
                        prefs.get_time_units_in_secs();

                    if ui
                        .add(egui::DragValue::new(&mut display_velocity))
                        .changed()
                    {
                        // 3. If the user changed the value, reverse the math and save it back
                        self.snapshot.object_velocity = (display_velocity *
                            prefs.get_time_units_in_secs() /
                            prefs.get_length_units_in_meters())
                            as _; // Uses '_' to auto-cast back to the original type
                    }
                });
                ui.checkbox(&mut self.initializing, "Set as initializing")
                    .on_hover_text(
                        "When ticked, the status of the distance sensor is set as '0x00', meaning \
                         that the sensor is still initializing. When left unticked it outputs \
                         '0x82', meaning the sensor is running with no problems.",
                    );
                if self.initializing {
                    self.snapshot.status = 0x00;
                } else {
                    self.snapshot.status = 0x82
                }
            });

        // Move this to the end so IPC gets the updated snapshots from this frame
        self.state(ipc, self.snapshot);
    }

    /// Writes `snapshot` into `ipc.readings` for this port
    pub fn state(&self, ipc: &mut AppIpc, snapshot: DistanceSnapshot) {
        ipc.readings.snapshots[self.port] = snapshot.into();
    }
}
