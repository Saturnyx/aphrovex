pub struct PrefWindow {
    pub open:  bool,
    pub prefs: Preferences,
}

pub struct Preferences {
    units: UnitPrefs,
}

pub struct UnitPrefs {
    length:      LengthUnits,
    time:        TimeUnits,
    angle:       AngleUnits,
}

#[derive(PartialEq, Debug)]
pub enum LengthUnits {
    Meters,
    Inches,
    Feet,
    Centimeters,
}

#[derive(PartialEq, Debug)]
pub enum TimeUnits {
    Milliseconds,
    Seconds,
    Minutes,
}

#[derive(PartialEq, Debug)]
pub enum AngleUnits {
    Radians,
    Degrees,
}

impl Default for PrefWindow {
    fn default() -> Self {
        Self {
            open:  false,
            prefs: Preferences {
                units: UnitPrefs {
                    length:      LengthUnits::Inches,
                    time:        TimeUnits::Seconds,
                    angle:       AngleUnits::Degrees,
                },
            },
        }
    }
}

impl PrefWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Preferences")
            .open(&mut self.open)
            .collapsible(true)
            .resizable(true)
            .show(ctx, |ui| {
                let length_label = egui::Label::new("Length Units:");
                let time_label = egui::Label::new("Time Units:");
                let angle_label = egui::Label::new("Angle Units:");
                ui.add(length_label);
                egui::ComboBox::from_id_salt("length_units")
                    .selected_text(format!("{:?}", self.prefs.units.length))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.prefs.units.length,
                            LengthUnits::Meters,
                            "Meters",
                        );
                        ui.selectable_value(
                            &mut self.prefs.units.length,
                            LengthUnits::Inches,
                            "Inches",
                        );
                        ui.selectable_value(
                            &mut self.prefs.units.length,
                            LengthUnits::Feet,
                            "Feet",
                        );
                        ui.selectable_value(
                            &mut self.prefs.units.length,
                            LengthUnits::Centimeters,
                            "Centimeters",
                        );
                    });
                ui.add(time_label);
                egui::ComboBox::from_id_salt("time_units")
                    .selected_text(format!("{:?}", self.prefs.units.time))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.prefs.units.time,
                            TimeUnits::Milliseconds,
                            "Milliseconds",
                        );
                        ui.selectable_value(
                            &mut self.prefs.units.time,
                            TimeUnits::Seconds,
                            "Seconds",
                        );
                        ui.selectable_value(
                            &mut self.prefs.units.time,
                            TimeUnits::Minutes,
                            "Minutes",
                        );
                    });
                ui.add(angle_label);
                egui::ComboBox::from_id_salt("angle_units")
                    .selected_text(format!("{:?}", self.prefs.units.angle))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.prefs.units.angle,
                            AngleUnits::Radians,
                            "Radians",
                        );
                        ui.selectable_value(
                            &mut self.prefs.units.angle,
                            AngleUnits::Degrees,
                            "Degrees",
                        );
                    });
            });
    }
}

impl Preferences {
    /// degrees -> 360
    /// radians -> 2*pi
    pub fn get_rotation_units(&self) -> f32 {
        match self.units.angle {
            AngleUnits::Degrees => 360.0,
            AngleUnits::Radians => 2.0 * std::f32::consts::PI,
        }
    }

    pub fn get_rotation_units_as_string(&self) -> String {
        match self.units.angle {
            AngleUnits::Degrees => "degrees".to_string(),
            AngleUnits::Radians => "radians".to_string(),
        }
    }

    pub fn get_time_units(&self) -> f32 {
        match self.units.time {
            TimeUnits::Milliseconds => 1000.0,
            TimeUnits::Seconds => 1.0,
            TimeUnits::Minutes => 1.0 / 60.0,
        }
    }

    pub fn get_time_units_as_string(&self) -> String {
        match self.units.time {
            TimeUnits::Milliseconds => "ms".to_string(),
            TimeUnits::Seconds => "sec".to_string(),
            TimeUnits::Minutes => "min".to_string(),
        }
    }
}
