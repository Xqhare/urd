
mod gui;
mod journal_entries;
mod settings;
mod paths;
mod startup;

fn main() {
    let settings = startup::startup_appstate();
    gui::gui_startup(settings);
}
