use settings::Settings;


mod settings;
mod gui;
mod journal_entries;

fn main() {
    let settings = Settings::default();
    gui::gui_startup(settings);
}
