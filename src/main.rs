#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
use error::Error;

mod error;
mod gui;
mod journal_entries;
mod moods;
mod paths;
mod render;
mod search;
mod settings;
mod startup;
mod tips_and_tricks;

fn main() {
    let mut settings = startup::startup_appstate();
    if settings.settings.automatic_backups {
        let pos_err = settings.journal.create_backup(
            &settings.settings,
            &settings.settings.custom_paths.backup_directory,
        );
        if pos_err.is_err() {
            settings.error = Error::new(
                pos_err.unwrap_err().to_string(),
                "Creating backup failed.".to_string(),
            );
        }
    }
    gui::gui_startup(settings);
}
