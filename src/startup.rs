use std::path::Path;

use crate::{
    error::Error,
    journal_entries::Journal,
    paths::{APP_DIR, SETTINGS_FILE},
    settings::Settings,
};

pub struct StartupState {
    pub settings: Settings,
    pub journal: Journal,
    pub error: Error,
}

pub fn startup_appstate() -> StartupState {
    if Path::new(APP_DIR).exists() {
        let settings = startup_settings();
        if settings.is_err() {
            // first recoverable error; everything is fine
            let fallback_settings = Settings::default();
            let journal = Journal::load(&fallback_settings);
            if journal.is_err() {
                // second recoverable error; everything is fine - even though everything is on fire
                let fallback_journal = Journal::new(&fallback_settings);
                return StartupState {
                    settings: fallback_settings,
                    journal: fallback_journal,
                    error: Error::new(
                        journal.unwrap_err().to_string(),
                        "Loading journal failed.".to_string(),
                    ),
                };
            }
            StartupState {
                settings: fallback_settings,
                journal: journal.unwrap(),
                error: settings.unwrap_err(),
            }
        } else {
            let tmp_settings = settings.unwrap();
            let journal = Journal::load(&tmp_settings);
            if journal.is_err() {
                // first recoverable error; everything is fine
                let fallback_journal = Journal::new(&tmp_settings);
                return StartupState {
                    settings: tmp_settings,
                    journal: fallback_journal,
                    error: Error::new(
                        journal.unwrap_err().to_string(),
                        "Loading journal failed.".to_string(),
                    ),
                };
            } else {
                StartupState {
                    settings: tmp_settings,
                    journal: journal.unwrap(),
                    error: Error::default(),
                }
            }
        }
    } else {
        // Assume first time startup
        std::fs::create_dir(APP_DIR).unwrap();

        let settings = Settings::default();
        let journal = Journal::new(&settings);
        StartupState {
            settings,
            journal,
            error: Error::default(),
        }
    }
}

fn startup_settings() -> Result<Settings, Error> {
    if Path::new(SETTINGS_FILE).exists() {
        match Settings::load(SETTINGS_FILE) {
            Ok(settings) => Ok(settings),
            Err(e) => Err(Error::new(
                e.to_string(),
                "Loading settings failed.".to_string(),
            )),
        }
    } else {
        Ok(Settings::default())
    }
}
