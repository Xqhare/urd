use std::sync::{atomic::AtomicBool, Arc};

use crate::{
    journal_entries::{Journal, JournalEntry},
    settings::Settings,
};
use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{Align, FontId, ScrollArea, SidePanel, TextEdit, TopBottomPanel};

mod about;
mod help;
mod licenses;
mod settings;
mod main_page;
mod search_page;

const APP_NAME: &str = "Urd";

pub struct UrdState {
    journal: Journal,
    settings: Settings,
    editing_index: Option<usize>,
    search_mode: bool,
    show_error: bool,
    error_message: Option<String>,
    show_about_viewport: Arc<AtomicBool>,
    show_licenses_viewport: Arc<AtomicBool>,
    show_help_viewport: Arc<AtomicBool>,
}

impl Default for UrdState {
    fn default() -> Self {
        let settings = Settings::default();
        UrdState {
            journal: Journal::new(&settings),
            settings,
            editing_index: None,
            error_message: None,
            // default false
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
            search_mode: false,
            show_error: false,
        }
    }
}

impl UrdState {
    // TODO: if settings have been found, there are probably journal entries to check for
    pub fn new(settings: Settings) -> Self {
        UrdState {
            journal: Journal::new(&settings),
            settings,
            editing_index: None,
            error_message: None,
            // default false
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
            search_mode: false,
            show_error: false,
        }
    }
}

impl App for UrdState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.main_top_panel(ctx, frame);
        if self.search_mode {
            self.search_page(ctx, frame);
        } else {
            self.main_page(ctx, frame);
        }
        if self
            .settings
            .show_settings_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.settings_viewport_startup(ctx);
        }
        if self
            .show_about_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.about_viewport_startup(ctx);
        }
        if self
            .show_licenses_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.licenses_viewport_startup(ctx);
        }
        if self
            .show_help_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.help_viewport_startup(ctx);
        }
    }
}

impl UrdState {
    fn main_top_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                ui.add(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.button("Settings").clicked() {
                            self.settings
                                .show_settings_viewport
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                        if ui.button("About").clicked() {
                            self.show_about_viewport
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                        if ui.button("Licenses").clicked() {
                            self.show_licenses_viewport
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                        if ui.button("Help").clicked() {
                            self.show_help_viewport
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    }).response
                    
                });
                ui.add_space(self.settings.size.size[0] / 3.0);
                if self.show_error {
                    ui.add(|ui: &mut Ui| {
                        ui.horizontal(|ui: &mut Ui| {
                            ui.label("Error:");
                            ui.label(self.error_message.as_ref().unwrap());
                            if ui.button("Dismiss").clicked() {
                                self.show_error = false;
                                self.error_message = None;
                            }
                        }).response
                    });
                };
                    
            });
        });
    }

    fn delete_journal_entry(&mut self) {
        if let Some(index) = self.editing_index {
            self.journal.entries.remove(index);
            self.editing_index = None;
        } else {
            self.journal.current_entry = JournalEntry::new(&self.settings);
        }
    }

    fn save_journal_entry(&mut self) {
        if let Some(index) = self.editing_index {
            self.journal.entries[index] = self.journal.current_entry.clone();
            self.journal.current_entry = JournalEntry::new(&self.settings);
            self.editing_index = None;
        } else {
            self.journal
            .entries
            .push_front(self.journal.current_entry.clone());
            self.journal.current_entry = JournalEntry::new(&self.settings);
        }
        
    }
}

pub fn gui_startup(settings: Settings) {
    let size: Vec2 = Vec2 {
        x: settings.size.size[0],
        y: settings.size.size[1],
    };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(
        APP_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::<UrdState>::new(UrdState::new(settings)))),
    )
    .expect("E 01");
}
