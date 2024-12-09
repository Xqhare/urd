use std::{str::FromStr, sync::{atomic::AtomicBool, Arc}};

use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{FontId, ScrollArea, SidePanel, TextEdit, TopBottomPanel};
use crate::settings::Settings;

mod settings;
mod about;
mod licenses;
mod help;

const APP_NAME: &str = "Urd";

pub struct UrdState {
    settings: Settings,
    current_journal_entry: String,
    current_journal_entry_title: String,
    show_about_viewport: Arc<AtomicBool>,
    show_licenses_viewport: Arc<AtomicBool>,
    show_help_viewport: Arc<AtomicBool>,
}

impl Default for UrdState {
    fn default() -> Self {
        UrdState {
            settings: Settings::default(),
            current_journal_entry: String::new(),
            current_journal_entry_title: String::new(),
            // default false
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl UrdState {
    // TODO: if settings have been found, there are probably journal entries to check for
    pub fn new(settings: Settings) -> Self {
        UrdState {
            settings,
            current_journal_entry: String::from_str("Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.").unwrap(),
            current_journal_entry_title: String::from_str("2024-12-09").unwrap(),
            // default false
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
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
        self.main_top_panel(ctx, frame);
        self.main_side_panel(ctx, frame);
        // Remember, central panel last
        self.main_central_panel(ctx, frame);
        
    }

    fn main_top_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                    if ui.button("Settings").clicked() {
                        self.settings.show_settings_viewport.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    if ui.button("About").clicked() {
                        self.show_about_viewport.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    if ui.button("Licenses").clicked() {
                        self.show_licenses_viewport.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
                    if ui.button("Help").clicked() {
                        self.show_help_viewport.store(true, std::sync::atomic::Ordering::Relaxed);
                    }
            });
        });
        if self.settings.show_settings_viewport.load(std::sync::atomic::Ordering::Relaxed) {
            self.settings_viewport_startup(ctx);
        }
        if self.show_about_viewport.load(std::sync::atomic::Ordering::Relaxed) {
            self.about_viewport_startup(ctx);
        }
        if self.show_licenses_viewport.load(std::sync::atomic::Ordering::Relaxed) {
            self.licenses_viewport_startup(ctx);
        }
        if self.show_help_viewport.load(std::sync::atomic::Ordering::Relaxed) {
            self.help_viewport_startup(ctx);
        }
    }

    fn main_side_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let font = {
            if self.settings.font.monospace {
                FontId::monospace(self.settings.font.size)
            } else {
                FontId::proportional(self.settings.font.size)
            }
        };
        SidePanel::left("entry_browser").default_width(self.settings.size.side_panel_width).show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                for n in 0..25 {
                    let entry_reaction = ui.add(|ui: &mut Ui| {
                        let group = ui.group(|ui: &mut Ui| {
                            let mut tmp_title_str = format!("Entry {}", n);
                            let mut tmp_body_str = "Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet. Nisi anim cupidatat excepteur officia. Reprehenderit nostrud nostrud ipsum Lorem est aliquip amet voluptate voluptate dolor minim nulla est proident. Nostrud officia pariatur ut officia. Sit irure elit esse ea nulla sunt ex occaecat reprehenderit commodo officia dolor Lorem duis laboris cupidatat officia voluptate. Culpa proident adipisicing id nulla nisi laboris ex in Lorem sunt duis officia eiusmod. Aliqua reprehenderit commodo ex non excepteur duis sunt velit enim. Voluptate laboris sint cupidatat ullamco ut ea consectetur et est culpa et culpa duis.".to_string();
                            ui.add_enabled(false, |ui: &mut Ui| {
                                TextEdit::singleline(&mut tmp_title_str).frame(false).desired_width(f32::INFINITY).text_color(self.settings.font.text_colour).font(font.clone()).show(ui).response
                            });
                            ui.add_enabled(false, |ui: &mut Ui| {
                                TextEdit::multiline(&mut tmp_body_str).frame(false).desired_width(f32::INFINITY).text_color(self.settings.font.text_colour).font(font.clone()).show(ui).response
                            })
                            
                        });
                        group.response
                    });
                    // TODO: open this journal entry if clicked
                    if entry_reaction.interact(egui::Sense::click()).clicked() {
                        println!("Entry reaction! {} clicked", n);
                    }
                }
                ui.separator();
            });
        });
    }

    fn main_central_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let font = {
            if self.settings.font.monospace {
                FontId::monospace(self.settings.font.size)
            } else {
                FontId::proportional(self.settings.font.size)
            }
        };
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            self.central_panel_menu(ui);
            
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                let title = TextEdit::singleline(&mut self.current_journal_entry_title).desired_width(f32::INFINITY).text_color(self.settings.font.text_colour).font(font.clone()).show(ui);
                let text_edit = TextEdit::multiline(&mut self.current_journal_entry).desired_width(f32::INFINITY).lock_focus(true).text_color(self.settings.font.text_colour).font(font.clone()).show(ui);
                if self.settings.continuous_saving {
                    if title.response.changed() || text_edit.response.changed() {
                        // TODO: save journal entry
                        // this saves every frame
                        println!("testing changed");
                    }
                } else {
                    if title.response.lost_focus() || text_edit.response.lost_focus() {
                        // TODO: save journal entry
                        // this saves only if the focus leaves the text box
                        println!("testing lost focus");
                    }
                }
                
            })
        });
    }

    fn central_panel_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.group(|ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Text Colour: ");
                    ui.color_edit_button_srgba(&mut self.settings.font.text_colour);
                })
            });
            ui.group(|ui: &mut Ui| {
                ui.label("Font Size: ");
                ui.add(egui::Slider::new(&mut self.settings.font.size, 8.0..=48.0));
            });
            ui.group(|ui: &mut Ui| {
                ui.checkbox(&mut self.settings.font.monospace, "Monospace");
            });
            let _ = ui.button("Save");
        });
    }
}

pub fn gui_startup(settings: Settings) {
    let size: Vec2 = Vec2 { x: settings.size.size[0], y: settings.size.size[1] };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(
        APP_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::<UrdState>::new(UrdState::new(settings)))),
    ).expect("E 01");
}
