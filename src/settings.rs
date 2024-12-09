use std::sync::{atomic::AtomicBool, Arc};

use eframe::egui::Color32;

// TODO: get from config
// TODO: save to config
pub struct Settings {
    pub size: SizeSettings,
    pub font: FontSettings,
    pub continuous_saving: bool,
    pub show_settings_viewport: Arc<AtomicBool>,
}

impl Default for Settings {
    
    // TODO: sensible defaults
    fn default() -> Self {
        Self {
            size: SizeSettings::default(),
            font: FontSettings::default(),
            // default true
            continuous_saving: true,
            // default false
            show_settings_viewport: Arc::new(AtomicBool::new(false)),
        }
    }
}

pub struct FontSettings {
    pub size: f32,
    pub text_colour: Color32,
    pub monospace: bool,
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            size: 16.0,
            text_colour: Color32::from_rgba_premultiplied(0, 0, 0, 255),
            monospace: false,
        }
    }
}

pub struct SizeSettings {
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
