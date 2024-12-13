use std::path::Path;

use crate::{paths::{APP_DIR, SETTINGS_FILE}, settings::Settings};


pub fn startup_appstate() -> Settings {
    if Path::new(APP_DIR).exists() {
        if Path::new(SETTINGS_FILE).exists() {
            let out = Settings::deserialize(SETTINGS_FILE);
            match out {
                Ok(s) => s,
                Err(_) => {
                    Settings::default()
                }
            }
        } else {
            Settings::default()
        }
    } else {
        // Assume first time startup
        std::fs::create_dir(APP_DIR).unwrap();
        Settings::default()
    }
}
