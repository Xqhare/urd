use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{ScrollArea, SidePanel};
use settings::Settings;

mod settings;

const APP_NAME: &str = "Urd";

pub struct UrdState {
    settings: Settings,

}

impl Default for UrdState {
    fn default() -> Self {
        UrdState {
            settings: Settings::default(),
        }
    }
}

impl UrdState {
    pub fn new(settings: Settings) -> Self {
        UrdState {
            settings,
        }
    }
}

impl App for UrdState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        SidePanel::left("entry_browser").default_width(self.settings.side_panel_width).show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.label("Hello World from SidePanel!");
                ui.label("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.");
            });
        });
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.label("Hello World from CentralPanel!");
                ui.label("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.");
            })
        });
    }
}

pub fn main() {
    let settings = Settings::default();
    let size: Vec2 = Vec2 { x: settings.size[0], y: settings.size[1] };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(
        APP_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::<UrdState>::new(UrdState::new(settings)))),
    ).expect("E 01");
}
