use std::path::Path;

use crate::{journal_entries::Journal, paths::{APP_DIR, SETTINGS_FILE}, settings::Settings};

pub struct StartupState {
    pub settings: Settings,
    pub journal: Journal,
}

pub fn startup_appstate() -> StartupState {
    let mut settings = Settings::default();
    let mut journal = Journal::new(&settings);
    if Path::new(APP_DIR).exists() {
        settings = startup_settings();
        // TODO: load journal
    } else {
        // Assume first time startup
        std::fs::create_dir(APP_DIR).unwrap();
    };

    StartupState {
        settings,
        journal,
    }
}

fn startup_settings() -> Settings {
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
}
