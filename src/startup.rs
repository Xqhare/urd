use std::path::Path;

use crate::settings::Settings;

const APP_DIR: &str = "urd_data/";

pub fn startup_appstate() -> Settings {
    if Path::new(APP_DIR).exists() {
        println!("App state already exists");
    } else {
        std::fs::create_dir(APP_DIR).unwrap();
    }
    let settings = Settings::default();
    settings
}
