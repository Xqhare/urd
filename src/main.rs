use settings::Settings;

mod gui;
mod journal_entries;
mod settings;

fn main() {
    let settings = Settings::default();
    gui::gui_startup(settings);
}
