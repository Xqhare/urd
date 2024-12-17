
use crate::{
    error::Error, journal_entries::Journal, render::Render, settings::Settings, startup::StartupState
};
use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{Align, TextEdit, TopBottomPanel};

mod about;
mod help;
mod licenses;
mod settings;
mod main_page;
mod main_page_side_panel;
mod search_page;

const APP_NAME: &str = "Urd";

pub struct UrdState {
    journal: Journal,
    settings: Settings,
    error: Error,
    render: Render,
    // misc
    editing_index: Option<usize>,
    search_mode: bool,
    settings_backup: Option<Settings>,
}

impl Default for UrdState {
    fn default() -> Self {
        let settings = Settings::default();
        UrdState {
            journal: Journal::new(&settings),
            settings,
            error: Error::default(),
            render: Render::default(),
            editing_index: None,
            settings_backup: None,
            // default false
            search_mode: false,
        }
    }
}

impl UrdState {
    // TODO: if settings have been found, there are probably journal entries to check for
    pub fn new(settings: Settings, journal: Journal) -> Self {
        UrdState {
            journal,
            settings,
            error: Error::default(),
            render: Render::default(),
            editing_index: None,
            settings_backup: None,
            // default false
            search_mode: false,
        }
    }
}

impl App for UrdState {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if self.settings.password.password != "" && !self.settings.password.unlocked_with_password {
            self.protected_mode(ctx, frame);
        } else {
            self.normal_mode(ctx, frame);
        }
    }
}

impl UrdState {

    fn protected_mode(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.add_space(ui.available_height() / 3.0);

            ui.add_sized([ui.available_width() / 1.33, 150.0], |ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.add_space(ui.available_width() / 3.0);
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.group(|ui: &mut Ui| {
                            ui.heading("Welcome to Urd");
                            ui.vertical_centered(|ui: &mut Ui| {
                                let tmp = ui.add(TextEdit::singleline(&mut self.settings.password.password_input).password(true).hint_text("Please enter your password here").horizontal_align(Align::Center));
                                tmp.request_focus();
                                if tmp.changed() {
                                    if self.settings.password.password_input.len() >= self.settings.password.password.len() {
                                        if self.settings.password.password == self.settings.password.password_input {
                                            self.settings.password.unlocked_with_password = true;
                                            self.settings.password.password_input = "".to_string();
                                            self.error = Error::default();
                                        } else {
                                            self.error = Error::new("Incorrect password");
                                        }
                                    } else {
                                        self.error = Error::default();
                                    }
                                }
                                if self.error.show_error {
                                    ui.add(|ui: &mut Ui| {
                                        ui.label(self.error.error_message.as_ref().unwrap())
                                    });
                                };
                            });
                        });
                    });
                }).response
            })
        });
    }

    fn normal_mode(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.main_top_panel(ctx, frame);
        if self.search_mode {
            self.search_page(ctx, frame);
        } else {
            self.main_page(ctx, frame);
        }
        
        if self
            .render
            .viewports
            .show_about_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.about_viewport_startup(ctx);
        }
        if self
            .render
            .viewports
            .show_licenses_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.licenses_viewport_startup(ctx);
        }
        if self
            .render
            .viewports
            .show_help_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.help_viewport_startup(ctx);
        }
    }

    fn main_top_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                ui.add(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        ui.menu_button("Urd", |ui: &mut Ui| {
                            if ui.button("Settings").clicked() {
                                if self.settings.show_settings_viewport {
                                    self.settings.show_settings_viewport = false;
                                    self.settings_backup = None;
                                } else {
                                    self.settings.show_settings_viewport = true;
                                    self.settings_backup = Some(self.settings.clone());
                                }
                            }
                            if ui.button("About").clicked() {
                                self.render.viewports.show_about_viewport
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                            }
                            if ui.button("Licenses").clicked() {
                                self.render.viewports.show_licenses_viewport
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                            }
                            if ui.button("Help").clicked() {
                                self.render.viewports.show_help_viewport
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                            }
                        });
                        ui.menu_button("Journal", |ui: &mut Ui| {
                            if ui.button("Search").clicked() {
                                if self.search_mode {
                                    self.search_mode = false;
                                } else {
                                    self.search_mode = true;
                                }
                            }
                            if ui.button("Export").clicked() {
                            }
                            if ui.button("").clicked() {
                            }
                        });
                        if self.settings.password.password != "" {
                            if ui.button("Lock Urd").clicked() {
                                self.settings.password.unlocked_with_password = false;
                            }
                        }
                    }).response
                });
                ui.add_space(ui.available_width() / 2.5);
                if self.error.show_error {
                    ui.add(|ui: &mut Ui| {
                        ui.horizontal_wrapped(|ui: &mut Ui| {
                            ui.label("Error:");
                            ui.label(self.error.error_message.as_ref().unwrap());
                            if ui.button("Dismiss").clicked() {
                                self.error = Error::default();
                            }
                        }).response
                    });
                };
                    
            });
        });
    }

    fn delete_journal_entry(&mut self) {
        /* if let Some(index) = self.editing_index {
            self.journal.entries.remove(index);
            self.editing_index = None;
        } else {
            self.journal.current_entry = JournalEntry::new(&self.settings);
        } */
    }

    fn save_journal_entry(&mut self) {
        /* if let Some(index) = self.editing_index {
            self.journal.entries[index] = self.journal.current_entry.clone();
            self.journal.current_entry = JournalEntry::new(&self.settings);
            self.editing_index = None;
        } else {
            self.journal
            .entries
            .push_front(self.journal.current_entry.clone());
            self.journal.current_entry = JournalEntry::new(&self.settings);
        } */
        
    }
}

pub fn gui_startup(startup_state: StartupState) {
    let size: Vec2 = Vec2 {
        x: startup_state.settings.size.size[0],
        y: startup_state.settings.size.size[1],
    };
    let mut native_options = NativeOptions::default();
    native_options.viewport.inner_size = Option::from(size);
    run_native(
        APP_NAME,
        native_options,
        Box::new(|_cc| Ok(Box::<UrdState>::new(UrdState::new(startup_state.settings, startup_state.journal)))),
    )
    .expect("E 01");
}
