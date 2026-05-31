use rand::Rng;
use roboscope_ipc::{
    cmd::{DeviceCommand, MotorCommand},
    snapshot::MotorSnapshot,
};

use crate::{ipc::AppIpc, prefs::Preferences};

const ROTATION_TO_TICKS: f32 = 4096.0;

pub struct MotorWindowState {
    pub id:       u64,
    pub open:     bool,
    pub port:     usize,
    pub snapshot: MotorSnapshot,
}

impl Default for MotorWindowState {
    fn default() -> Self {
        let mut rng = rand::rng();
        Self {
            id:       rng.next_u64(),
            open:     true,
            port:     0,
            snapshot: MotorSnapshot::default(),
        }
    }
}

impl MotorWindowState {
    pub fn show(&mut self, ctx: &egui::Context, ipc: &mut AppIpc, prefs: &Preferences) {
        let motor_state = self.state(ipc, self.snapshot);
        let mut port_num: i32 = self.port as i32 + 1; // slider_value: 1..=21
        let mut position =
            (self.snapshot.raw_position as f32) * prefs.get_rotation_units() / ROTATION_TO_TICKS; // convert back to degrees for display
        let mut veloc = (self.snapshot.raw_velocity as f32) * prefs.get_rotation_units() /
            ROTATION_TO_TICKS /
            prefs.get_time_units(); // convert back to user units per user time
        egui::Window::new("Motor")
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
                    ui.label(format!("Angle (in {})", prefs.get_rotation_units_as_string()));
                    if ui
                        .add(egui::DragValue::new(&mut position))
                        .on_hover_text(format!("in {}", prefs.get_rotation_units_as_string()))
                        .changed()
                    {
                        self.snapshot.raw_position =
                            (position * ROTATION_TO_TICKS / prefs.get_rotation_units()) as i32;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(format!(
                        "Velocity (in {}/{})",
                        prefs.get_rotation_units_as_string(),
                        prefs.get_time_units_as_string()
                    ));
                    if ui
                        .add(egui::DragValue::new(&mut veloc))
                        .on_hover_text(format!(
                            "in {}/{})",
                            prefs.get_rotation_units_as_string(),
                            prefs.get_time_units_as_string()
                        ))
                        .changed()
                    {
                        self.snapshot.raw_velocity =
                            (veloc * ROTATION_TO_TICKS / prefs.get_rotation_units() *
                                prefs.get_time_units()) as i32;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Flags");
                    ui.label(format!("{:?}", self.snapshot.flags));
                });

                ui.horizontal(|ui| {
                    ui.label("Faults");
                    ui.label(format!("{:?}", self.snapshot.faults));
                });

                ui.horizontal(|ui| {
                    ui.label("Temperature (°C)");
                    ui.add(egui::Slider::new(&mut self.snapshot.temperature, 0..=150));
                });

                ui.horizontal(|ui| {
                    ui.label("Current (mA)");
                    ui.add(egui::DragValue::new(&mut self.snapshot.current));
                });

                ui.horizontal(|ui| {
                    ui.label("Power (W)");
                    ui.add(egui::DragValue::new(&mut self.snapshot.power).speed(0.1));
                });

                ui.horizontal(|ui| {
                    ui.label("Torque (Nm)");
                    ui.add(egui::DragValue::new(&mut self.snapshot.torque).speed(0.01));
                });

                ui.horizontal(|ui| {
                    ui.label("Efficiency (%)");
                    ui.add(egui::Slider::new(&mut self.snapshot.efficiency, 0.0..=100.0));
                });

                ui.horizontal(|ui| {
                    ui.label("Applied Voltage (mV)");
                    ui.add(egui::DragValue::new(&mut self.snapshot.applied_voltage));
                });

                ui.add(egui::Label::new(format!("motor: {:?}", motor_state)));
            });
    }

    /// Writes `snapshot` into `ipc.readings` for this port, then returns the
    /// current motor command from `ipc.output` for the same port.
    pub fn state(&self, ipc: &mut AppIpc, snapshot: MotorSnapshot) -> MotorCommand {
        ipc.readings.snapshots[self.port] = snapshot.into();

        if let DeviceCommand::Motor(motor) = ipc.output[self.port] {
            motor
        } else {
            MotorCommand::default()
        }
    }
}
