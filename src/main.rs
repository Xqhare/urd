use settings::Settings;


mod settings;
mod gui;

fn main() {
    let settings = Settings::default();
    gui::gui_startup(settings);
}
