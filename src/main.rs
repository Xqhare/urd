use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::SidePanel;

const APP_NAME: &str = "Urd";

pub struct UrdState {}

impl Default for UrdState {
    fn default() -> Self {
        UrdState {}
    }
}

impl App for UrdState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        SidePanel::left("entry_browser").default_width(500.0).show(ctx, |ui: &mut Ui| {
            ui.label("Hello World from SidePanel!");
            ui.label("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.");
        });
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Hello World from CentralPanel!");
        });
    }
}

pub fn main() {
    let size: Vec2 = Vec2 { x: 800.0, y: 400.0 };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(
        APP_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::<UrdState>::default())),
    ).expect("E 01");
}
