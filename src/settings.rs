use std::{path::Path, sync::{atomic::AtomicBool, Arc}};

use eframe::egui::Color32;
use horae::TimeZone;
use nabu::XffValue;

use crate::paths::SETTINGS_FILE;

// TODO: get from config
// TODO: save to config
#[derive(Clone, Debug)]
pub struct Settings {
    pub size: SizeSettings,
    pub font: FontSettings,
    pub folder_structure: bool,
    pub show_settings_viewport: Arc<AtomicBool>,
    pub timezone: Option<TimeZone>,
}

impl Default for Settings {
    // TODO: sensible defaults
    fn default() -> Self {
        Self {
            size: SizeSettings::default(),
            font: FontSettings::default(),
            timezone: None,
            // default true
            folder_structure: true,
            // default false
            show_settings_viewport: Arc::new(AtomicBool::new(false)),
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

        serialized.insert("continuous_saving", XffValue::from(self.folder_structure));
        
        let timezone = match &self.timezone {
            Some(tz) => XffValue::from(tz.to_string()),
            None => XffValue::Null,
        };

        serialized.insert("timezone", timezone);

        XffValue::from(serialized)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let serialized = self.serialize();
        println!("{:?}", serialized);
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
        let continuous_saving = deserialized.get("continuous_saving").unwrap().into_boolean().unwrap();
        let timezone = match deserialized.get("timezone").unwrap() {
            XffValue::Null => None,
            _ => Some(TimeZone::from(deserialized.get("timezone").unwrap().into_string().unwrap())),
        };
        Ok(Settings {
            size,
            font,
            timezone,
            folder_structure: continuous_saving,
            show_settings_viewport: Arc::new(AtomicBool::new(false)),
        })
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
            text_colour: Color32::from_rgba_premultiplied(255, 255, 255, 255),
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
            size: [1000.0, 500.0],
            side_panel_width: 300.0,
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
