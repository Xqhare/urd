use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{ScrollArea, SidePanel, TextEdit};
use crate::settings::Settings;

const APP_NAME: &str = "Urd";

pub struct UrdState {
    settings: Settings,
    current_journal_entry: String,
    current_journal_entry_title: String,
}

impl Default for UrdState {
    fn default() -> Self {
        UrdState {
            settings: Settings::default(),
            current_journal_entry: String::new(),
            current_journal_entry_title: String::new(),
        }
    }
}

impl UrdState {
    // TODO: if settings have been found, there are probably journal entries to check for
    pub fn new(settings: Settings) -> Self {
        UrdState {
            settings,
            current_journal_entry: String::new(),
            current_journal_entry_title: String::new(),
        }
    }
}

impl App for UrdState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.main_page(ctx, frame);
    }
}

impl UrdState {
    fn main_page(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.main_side_panel(ctx, frame);
        // Remember, central panel last
        self.main_central_panel(ctx, frame);
        
    }

    fn main_side_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        SidePanel::left("entry_browser").default_width(self.settings.side_panel_width).show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {

                for n in 0..25 {

                    let entry_reaction = ui.group(|ui| {
                        ui.separator();
                        let title = ui.label(format!("Entry {}", n));
                        let body = ui.label("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.");
                    }).response;
                    if entry_reaction.clicked() {
                        println!("Entry {} clicked", n);
                    }
                }
                ui.separator();
            });
        });
    }

    fn main_central_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                let titel = TextEdit::singleline(&mut self.current_journal_entry_title).desired_width(f32::INFINITY).show(ui);
                let text_edit = TextEdit::multiline(&mut self.current_journal_entry).min_size(ui.available_size()).show(ui);
            })
        });
    }
}

pub fn gui_startup(settings: Settings) {
    let size: Vec2 = Vec2 { x: settings.size[0], y: settings.size[1] };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(
        APP_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::<UrdState>::new(UrdState::new(settings)))),
    ).expect("E 01");
}
