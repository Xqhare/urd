use error::Error;

mod gui;
mod journal_entries;
mod settings;
mod paths;
mod error;
mod startup;
mod render;
mod search;

fn main() {
    let mut settings = startup::startup_appstate();
    if settings.settings.automatic_backups {
        let pos_err = settings.journal.create_backup();
        if pos_err.is_err() {
            settings.error = Error::new(pos_err.unwrap_err().to_string(), "Creating backup failed.".to_string());
        }
    }
    gui::gui_startup(settings);
}
