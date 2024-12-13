use settings::Settings;

mod gui;
mod journal_entries;
mod settings;
mod startup;

fn main() {
    let _ = startup::startup_appstate();
    let settings = Settings::default();
    gui::gui_startup(settings);
}
