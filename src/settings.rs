use std::path::Path;

use eframe::egui::Color32;
use horae::TimeZone;
use nabu::XffValue;

use crate::paths::SETTINGS_FILE;

const MAIN_WINDOW_DEFAULT_SIZE: [f32; 2] = [1000.0, 600.0];
const SIDE_PANEL_DEFAULT_WIDTH: f32 = 300.0;
pub const MAX_FONT_SIZE: f32 = 100.0;
pub const MIN_FONT_SIZE: f32 = 2.0;
pub const MAX_WINDOW_SIZE: [f32; 2] = [3_000.0, 3_000.0];
pub const MAX_SIDE_PANEL_WIDTH: f32 = 2_000.0;

// TODO: get from config
// TODO: save to config
#[derive(Clone, Debug)]
pub struct Settings {
    pub size: SizeSettings,
    pub font: FontSettings,
    pub password: Password,
    pub timezone: TimezoneStore,
    pub gui: Gui,
    // Not part of persistent state
    pub show_settings_viewport: bool,
    pub overwrite_window_size: bool,
    pub overwrite_window_size_store: [String; 2],
    pub overwrite_side_panel_width: bool,
    pub overwrite_side_panel_width_store: String,
}

impl Default for Settings {
    // TODO: sensible defaults
    fn default() -> Self {
        let size = SizeSettings::default();
        Self {
            overwrite_window_size_store: [size.size[0].to_string(), size.size[1].to_string()],
            overwrite_side_panel_width_store: size.side_panel_width.to_string(),
            size,
            // default default/None
            font: FontSettings::default(),
            timezone: TimezoneStore::default(),
            password: Password::default(),
            gui: Gui::default(),
            // default true
            // default false
            show_settings_viewport: false,
            overwrite_window_size: false,
            overwrite_side_panel_width: false,
        }
    }
}

impl Settings {
    fn serialize(&self) -> XffValue {
        let mut serialized = nabu::Object::new();

        let size = self.size.serialize();
        let font = self.font.serialize();

        serialized.insert("size", size);
        serialized.insert("font", font);

        let timezone = XffValue::from(self.timezone.timezone.to_string());

        serialized.insert("timezone", timezone);

        let password = XffValue::from(self.password.password.clone());
        serialized.insert("password", password);

        let gui = self.gui.serialize();
        serialized.insert("gui", gui);

        XffValue::from(serialized)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let serialized = self.serialize();
        let out = nabu::serde::write(SETTINGS_FILE, serialized);
        match out {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        }
        
    }

    pub fn deserialize<P: AsRef<Path>>(path: P) -> Result<Settings, std::io::Error> {
        let deserialized = {
            let out = nabu::serde::read(path);
            match out {
                Ok(d) => d.into_object().unwrap(),
                Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
            }
        };
        let size = SizeSettings::deserialize(&deserialized.get("size").unwrap());
        let font = FontSettings::deserialize(&deserialized.get("font").unwrap());
        let tz = TimeZone::from(deserialized.get("timezone").unwrap().into_string().unwrap());
        let password = deserialized.get("password").unwrap().into_string().unwrap();
        let gui = Gui::deserialize(&deserialized.get("gui").unwrap());
        Ok(Settings {
            font,
            gui,
            timezone: TimezoneStore::new(tz),
            password: Password::new(password),
            show_settings_viewport: false,
            overwrite_window_size: false,
            overwrite_side_panel_width: false,
            overwrite_side_panel_width_store: size.side_panel_width.to_string(),
            overwrite_window_size_store: [size.size[0].to_string(), size.size[1].to_string()],
            size,
        })
    }
}

#[derive(Clone, Debug)]
pub struct Gui {
    pub file_marker_currently: FileMarker,
    pub file_marker_perfectly: FileMarker,
    pub file_marker_normally: FileMarker,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            file_marker_currently: FileMarker::new("|".to_string(), "|".to_string()),
            file_marker_perfectly: FileMarker::new("«".to_string(), "»".to_string()),
            file_marker_normally: FileMarker::new("<".to_string(), ">".to_string()),
        }
    }
}

impl Gui {
    pub fn serialize(&self) -> XffValue {
        let mut serialized = nabu::Object::new();
        serialized.insert("file_marker_currently", self.file_marker_currently.serialize());
        serialized.insert("file_marker_perfectly", self.file_marker_perfectly.serialize());
        serialized.insert("file_marker_normally", self.file_marker_normally.serialize());
        XffValue::from(serialized)
    }

    pub fn deserialize(serialized: &XffValue) -> Self {
        let file_marker_currently = FileMarker::deserialize(&serialized.into_object().unwrap().get("file_marker_currently").unwrap());
        let file_marker_perfectly = FileMarker::deserialize(&serialized.into_object().unwrap().get("file_marker_perfectly").unwrap());
        let file_marker_normally = FileMarker::deserialize(&serialized.into_object().unwrap().get("file_marker_normally").unwrap());
        Self {
            file_marker_currently,
            file_marker_perfectly,
            file_marker_normally,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileMarker {
    pub start: String,
    pub end: String,
}

impl FileMarker {
    pub fn new(start: String, end: String) -> Self {
        Self { start, end }
    }

    pub fn serialize(&self) -> XffValue {
        let mut serialized = nabu::Object::new();
        serialized.insert("start", XffValue::from(self.start.clone()));
        serialized.insert("end", XffValue::from(self.end.clone()));
        XffValue::from(serialized)
    }

    pub fn deserialize(serialized: &XffValue) -> Self {
        let start = serialized.into_object().unwrap().get("start").unwrap().into_string().unwrap();
        let end = serialized.into_object().unwrap().get("end").unwrap().into_string().unwrap();
        Self { start, end }
    }
}

#[derive(Clone, Debug)]
pub struct Password {
    pub password: String,
    pub password_input: String,
    pub new_password_input: [String; 2],
    pub unlocked_with_password: bool,
}

impl Default for Password {
    fn default() -> Self {
        Self {
            password: String::new(),
            password_input: String::new(),
            new_password_input: [String::new(), String::new()],
            unlocked_with_password: false,
        }
    }
}

impl Password {
    pub fn new(password: String) -> Self {
        Self {
            password,
            password_input: String::new(),
            new_password_input: [String::new(), String::new()],
            unlocked_with_password: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TimezoneStore {
    pub timezone: TimeZone,
    pub all_timezones_str: Vec<String>,
}

impl Default for TimezoneStore {
    fn default() -> Self {
        let all_timezones_str = {
            let mut out: Vec<String> = Vec::new();
            for tz in TimeZone::get_all() {
                out.push(tz.to_string());
            }
            out
        };
        Self {
            timezone: TimeZone::CoordinatedUniversalTime,
            all_timezones_str,
        }
    }
}

impl TimezoneStore {
    pub fn new(timezone: TimeZone) -> Self {
        let all_timezones_str = {
            let mut out: Vec<String> = Vec::new();
            for tz in TimeZone::get_all() {
                out.push(tz.to_string());
            }
            out
        };
        Self {
            timezone,
            all_timezones_str,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontSettings {
    pub size: f32,
    pub text_colour: Color32,
    pub monospace: bool,
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            size: 14.0,
            text_colour: Color32::from_rgba_premultiplied(127, 127, 127, 255),
            monospace: false,
        }
    }
}

impl FontSettings {
    pub fn deserialize(serialized: &XffValue) -> Self {
        let font = serialized.into_object().unwrap();
        // Unchecked conversion ok, because we know the value is derived from the smaller type
        let size = font.get("size").unwrap().into_number().unwrap().into_usize().unwrap() as f32;
        let text_colour = Color32::from_rgba_premultiplied(
            font.get("text_colour").unwrap().into_array().unwrap().get(0).unwrap().into_number().unwrap().into_usize().unwrap() as u8,
            font.get("text_colour").unwrap().into_array().unwrap().get(1).unwrap().into_number().unwrap().into_usize().unwrap() as u8,
            font.get("text_colour").unwrap().into_array().unwrap().get(2).unwrap().into_number().unwrap().into_usize().unwrap() as u8,
            font.get("text_colour").unwrap().into_array().unwrap().get(3).unwrap().into_number().unwrap().into_usize().unwrap() as u8,
        );
        let monospace = font.get("monospace").unwrap().into_boolean().unwrap();
        Self { size, text_colour, monospace }

    }

    pub fn serialize(&self) -> XffValue {
        let mut serialized = nabu::Object::new();
        serialized.insert("size", XffValue::from(self.size));
        serialized.insert("text_colour", XffValue::from(nabu::Array::from(self.text_colour.to_array().to_vec())));
        serialized.insert("monospace", XffValue::from(self.monospace));
        XffValue::from(serialized)
    }
}

#[derive(Clone, Debug)]
pub struct SizeSettings {
    /// First value is width, second is height
    pub size: [f32; 2],
    pub side_panel_width: f32,
}

impl Default for SizeSettings {
    fn default() -> Self {
        Self {
            size: MAIN_WINDOW_DEFAULT_SIZE,
            side_panel_width: SIDE_PANEL_DEFAULT_WIDTH,
        }
    }
}

impl SizeSettings {
    pub fn deserialize(serialized: &XffValue) -> Self {
        let size_array = serialized.into_object().unwrap().get("size").unwrap().into_array().unwrap();
        // the f64 can never be larger than f32, its derived from it!
        let size = [size_array.get(0).unwrap().into_number().unwrap().into_usize().unwrap() as f32, size_array.get(1).unwrap().into_number().unwrap().into_usize().unwrap() as f32];
        let side_panel_width = serialized.into_object().unwrap().get("side_panel_width").unwrap().into_number().unwrap().into_usize().unwrap() as f32;
        Self {
            size,
            side_panel_width,
        }
    }

    pub fn serialize(&self) -> XffValue {
        let mut serialized = nabu::Object::new();
        serialized.insert("size", XffValue::from(vec![XffValue::from(self.size[0]), XffValue::from(self.size[1])]));
        serialized.insert("side_panel_width", XffValue::from(self.side_panel_width));
        XffValue::from(serialized)
    }
}
