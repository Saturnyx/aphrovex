use egui_thematic::ThemeConfig;

pub struct PrefWindow {
    pub open:  bool,
    pub prefs: Preferences,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Preferences {
    pub units:              UnitPrefs,
    pub close_dialog:       bool, // whether close dialog is active
    pub transparent_window: f32,
    pub theme:              ThemeConfig,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UnitPrefs {
    length: LengthUnits,
    time:   TimeUnits,
    angle:  AngleUnits,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub enum LengthUnits {
    Meters,
    Inches,
    Feet,
    Centimeters,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub enum TimeUnits {
    Milliseconds,
    Seconds,
    Minutes,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub enum AngleUnits {
    Radians,
    Degrees,
    Rotations,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            units:              UnitPrefs {
                length: LengthUnits::Inches,
                time:   TimeUnits::Seconds,
                angle:  AngleUnits::Degrees,
            },
            close_dialog:       true,
            transparent_window: 0.2,
            theme:              ThemeConfig::dark_preset(),
        }
    }
}

impl Default for PrefWindow {
    fn default() -> Self {
        Self {
            open:  false,
            prefs: Preferences::default(),
        }
    }
}

impl PrefWindow {
    pub fn from_prefs(prefs: Preferences) -> Self { Self { open: false, prefs } }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Preferences")
            .open(&mut self.open)
            .collapsible(true)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("GUI");
                ui.checkbox(&mut self.prefs.close_dialog, "Enable close dialog");
                ui.add(egui::Slider::new(&mut self.prefs.transparent_window, 0.0..=1.0));
                egui::ComboBox::from_id_salt("theme")
                    .selected_text("Theme")
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::dark_preset(),
                            "Dark",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::light_preset(),
                            "Light",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::dracula_preset(),
                            "Dracula",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::gruvbox_dark_preset(),
                            "Gruvbox",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::monokai_preset(),
                            "Monokai",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::nord_preset(),
                            "Nord",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::one_dark_preset(),
                            "One Dark",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::solarized_dark_preset(),
                            "Solarized Dark",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::solarized_light_preset(),
                            "Solarized Light",
                        );
                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::tokyo_night_preset(),
                            "Tokyo Night",
                        );

                        ui.selectable_value(
                            &mut self.prefs.theme,
                            ThemeConfig::catppuccin_mocha_preset(),
                            "Catppuccin Mocha",
                        );
                    });
                ui.separator();
                ui.heading("Units");
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
                        ui.selectable_value(
                            &mut self.prefs.units.angle,
                            AngleUnits::Rotations,
                            "Rotations",
                        );
                    });
            });
    }
}

impl Preferences {
    /// degrees -> 360
    /// radians -> 2*pi
    pub fn get_rotation_units_in_rotations(&self) -> f32 {
        match self.units.angle {
            AngleUnits::Degrees => 360.0,
            AngleUnits::Radians => 2.0 * std::f32::consts::PI,
            AngleUnits::Rotations => 1.0,
        }
    }

    pub fn get_rotation_units_as_string(&self) -> String {
        match self.units.angle {
            AngleUnits::Degrees => "degrees".to_string(),
            AngleUnits::Radians => "radians".to_string(),
            AngleUnits::Rotations => "rotations".to_string(),
        }
    }

    pub fn get_time_units_in_secs(&self) -> f32 {
        match self.units.time {
            TimeUnits::Milliseconds => 1000.0,
            TimeUnits::Seconds => 1.0,
            TimeUnits::Minutes => 1.0 / 60.0,
        }
    }

    pub fn get_time_units_as_string(&self) -> String {
        match self.units.time {
            TimeUnits::Milliseconds => "ms".to_string(),
            TimeUnits::Seconds => "s".to_string(),
            TimeUnits::Minutes => "min".to_string(),
        }
    }

    pub fn get_length_units_in_meters(&self) -> f32 {
        match self.units.length {
            LengthUnits::Centimeters => 100.0,
            LengthUnits::Feet => 3.28084,
            LengthUnits::Inches => 39.3701,
            LengthUnits::Meters => 1.0,
        }
    }

    pub fn get_length_units_as_string(&self) -> String {
        match self.units.length {
            LengthUnits::Centimeters => "cm".to_string(),
            LengthUnits::Feet => "'".to_string(),
            LengthUnits::Inches => '"'.to_string(),
            LengthUnits::Meters => "m".to_string(),
        }
    }
}
