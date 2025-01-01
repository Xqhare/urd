
use crate::{
    error::Error, journal_entries::Journal, render::{Render, ShowFolder}, search::Search, settings::{NeededPath, Settings}, startup::StartupState
};
use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{panel::Side, Align, Id, Modal, Sides, TextEdit, TopBottomPanel};

mod about;
mod help;
mod licenses;
mod settings;
mod main_page;
mod main_page_side_panel;
mod search_page;
mod file_picker;

const APP_NAME: &str = "Urd";

pub struct UrdState {
    journal: Journal,
    settings: Settings,
    error: Error,
    render: Render,
    search: Search,
    // misc
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
            search: Search::default(),
            settings_backup: None,
        }
    }
}

impl UrdState {
    // TODO: if settings have been found, there are probably journal entries to check for
    pub fn new(settings: Settings, journal: Journal, error: Error) -> Self {
        UrdState {
            journal,
            settings,
            error,
            render: Render::default(),
            search: Search::default(),
            settings_backup: None,
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
                                let tmp = ui.add(TextEdit::singleline(&mut self.settings.password.password_input).password(true).hint_text("Please enter your password").horizontal_align(Align::Center));
                                tmp.request_focus();
                                if tmp.changed() {
                                    if self.settings.password.password_input.len() >= self.settings.password.password.len() {
                                        if self.settings.password.password == self.settings.password.password_input {
                                            self.settings.password.unlocked_with_password = true;
                                            self.settings.password.password_input = "".to_string();
                                            self.error = Error::default();
                                        } else {
                                            self.error = Error::new("Incorrect password".to_string(), "Unlocking Urd failed.".to_string());
                                            self.settings.password.password_input = "".to_string();
                                        }
                                    } else {
                                        self.error = Error::default();
                                    }
                                }
                                if self.error.show_error {
                                    self.error_modal(ui);
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
        if self.render.viewports.show_search_page {
            self.search_page(ctx, frame);
        } else if self.render.viewports.show_file_picker {
            self.file_picker(ctx, frame);
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
                ui.add_space(1.0);
                ui.add(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        ui.menu_button("Urd", |ui: &mut Ui| {
                            if ui.button("Settings").clicked() {
                                if self.render.viewports.show_settings_viewport {
                                    self.render.viewports.show_settings_viewport = false;
                                    self.settings_backup = None;
                                } else {
                                    self.render.viewports.show_settings_viewport = true;
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
                                if self.render.viewports.show_search_page {
                                    self.render.viewports.show_search_page = false;
                                } else {
                                    self.render.viewports.show_search_page = true;
                                }
                            }
                            if ui.button("Export").clicked() {
                                if self.settings.custom_paths.export_directory != "" {
                                    let pos_err = self.journal.export();
                                    if pos_err.is_err() {
                                        self.error = Error::new(pos_err.unwrap_err().to_string(), "Writing journal export to disk failed.".to_string());
                                    }
                                } else {
                                    self.settings.custom_paths.needed_path = Some(NeededPath::Export);
                                    self.render.viewports.show_file_picker = true;
                                    if !self.render.viewports.show_file_picker {
                                        let pos_err = self.journal.export();
                                        if pos_err.is_err() {
                                            self.error = Error::new(pos_err.unwrap_err().to_string(), "Writing journal export to disk failed.".to_string());
                                        }
                                    }
                                }
                            }
                            ui.menu_button("Backup", |ui: &mut Ui| {
                                if ui.button("Create").clicked() {
                                    if self.settings.custom_paths.backup_directory != "" {
                                        // Backup already set up
                                        let pos_err = self.journal.create_backup();
                                        if pos_err.is_err() {
                                            self.error = Error::new(pos_err.unwrap_err().to_string(), "Writing journal backup to disk failed.".to_string());
                                        }
                                    } else {
                                        self.settings.custom_paths.needed_path = Some(NeededPath::Backup);
                                        self.render.viewports.show_file_picker = true;
                                        if !self.render.viewports.show_file_picker {
                                            let pos_err = self.journal.create_backup();
                                            if pos_err.is_err() {
                                                self.error = Error::new(pos_err.unwrap_err().to_string(), "Writing journal backup to disk failed.".to_string());
                                            }
                                        }
                                    }

                                }
                                if ui.button("Restore").clicked() {
                                    // Restore_file is always empthy, I need to call the file
                                    // picker always
                                    self.settings.custom_paths.needed_path = Some(NeededPath::Restore);
                                    self.render.viewports.show_file_picker = true;
                                    if !self.render.viewports.show_file_picker {
                                        let pos_err = self.journal.restore_backup();
                                        if pos_err.is_err() {
                                            self.error = Error::new(pos_err.unwrap_err().to_string(), "Restoring journal from disk failed.".to_string());
                                        }
                                    }

                                }
                            });
                        });
                        ui.menu_button("Navigation", |ui: &mut Ui| {
                            if ui.button("Go to top level").clicked() {
                                self.render.show_folder = ShowFolder::All
                            };
                            if ui.button("Go back one level").clicked() {
                                self.go_back_one_level();
                            };
                        });
                        if self.settings.password.password != "" {
                            if ui.button("Lock Urd").clicked() {
                                self.settings.password.unlocked_with_password = false;
                            }
                        }
                        if self.render.viewports.show_file_picker {
                            if ui.button("Exit file picker").clicked() {
                                self.render.viewports.show_file_picker = false;
                            };
                        }
                    }).response
                });
                ui.add_space(ui.available_width() / 2.5);
                if self.error.show_error {
                    self.error_modal(ui);
                };
            });
            ui.add_space(1.0);
        });
    }

    fn error_modal(&mut self, ui: &mut Ui) {
        let error_modal = Modal::new(Id::new("Error Modal")).show(ui.ctx(), |ui: &mut Ui| {
            ui.vertical_centered_justified(|ui: &mut Ui| {
                ui.heading("Error");
            });
            ui.separator();
            ui.vertical_centered_justified(|ui: &mut Ui| {
                ui.label(self.error.error_context.as_ref().unwrap());
                ui.label("Reason: ");
                ui.label(self.error.error_message.as_ref().unwrap());
            });
            ui.separator();
            ui.scope(|ui: &mut Ui| {
                ui.vertical_centered_justified(|ui: &mut Ui| {
                    if ui.button("Dismiss").clicked() {
                        self.error.show_error = false;
                    }
                });
            });
        });
        if error_modal.should_close() {
            self.error.show_error = false;
        }
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
        Box::new(|_cc| Ok(Box::<UrdState>::new(UrdState::new(startup_state.settings, startup_state.journal, startup_state.error)))),
    )
    .expect("E 01");
}
