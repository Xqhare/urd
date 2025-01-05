use crate::{
    error::Error, journal_entries::Journal, moods::Mood, render::{Render, ShowFolder}, search::Search, settings::{NeededPath, Settings}, startup::StartupState
};
use eframe::{
    egui::{CentralPanel, Ui},
    epaint::Vec2,
    *,
};
use egui::{Align, Id, Modal, TextEdit, TopBottomPanel};

mod about;
mod file_picker;
mod help;
mod licenses;
mod main_page;
mod main_page_side_panel;
mod search_page;
mod settings;
mod important_days;
mod moods;

const APP_NAME: &str = "Urd";

pub struct StateStore {
    pub new_mood: Mood,
}

impl Default for StateStore {
    fn default() -> Self {
        Self {
            new_mood: Mood::default(),
        }
    }
}

pub struct UrdState {
    journal: Journal,
    settings: Settings,
    error: Error,
    render: Render,
    search: Search,
    settings_backup: Option<Settings>,
    state_store: StateStore,
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
            state_store: StateStore::default(),
        }
    }
}

impl UrdState {
    pub fn new(settings: Settings, journal: Journal, error: Error, first_run: bool) -> Self {
        if first_run {
            let settings = Settings::default();
            UrdState {
                journal: Journal::new(&settings),
                settings,
                error: Error::default(),
                render: Render::startup_default(),
                search: Search::default(),
                settings_backup: None,
                state_store: StateStore::default(),
            }
        } else {
            UrdState {
                journal,
                settings,
                error,
                render: Render::default(),
                search: Search::default(),
                settings_backup: None,
                state_store: StateStore::default(),
            }
        }
    }
}

impl App for UrdState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.settings.password.password != "" && !self.settings.password.unlocked_with_password {
            self.protected_mode(ctx);
        } else {
            self.normal_mode(ctx);
        }
    }
}

impl UrdState {
    fn protected_mode(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.add_space(ui.available_height() / 3.0);

            ui.add_sized([ui.available_width() / 1.33, 150.0], |ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.add_space(ui.available_width() / 3.0);
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.group(|ui: &mut Ui| {
                            ui.heading("Welcome to Urd");
                            ui.vertical_centered(|ui: &mut Ui| {
                                let tmp = ui.add(
                                    TextEdit::singleline(
                                        &mut self.settings.password.password_input,
                                    )
                                    .password(true)
                                    .hint_text("Please enter your password")
                                    .horizontal_align(Align::Center),
                                );
                                tmp.request_focus();
                                if tmp.changed() {
                                    if self.settings.password.password_input.len()
                                        >= self.settings.password.password.len()
                                    {
                                        if self.settings.password.password
                                            == self.settings.password.password_input
                                        {
                                            self.settings.password.unlocked_with_password = true;
                                            self.settings.password.password_input = "".to_string();
                                            self.error = Error::default();
                                        } else {
                                            self.error = Error::new(
                                                "Incorrect password".to_string(),
                                                "Unlocking Urd failed.".to_string(),
                                            );
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
                })
                .response
            })
        });
    }

    fn normal_mode(&mut self, ctx: &egui::Context) {
        self.main_top_panel(ctx);
        if self.render.view.show_important_days_page {
            self.important_days_page(ctx);
        } else if self.render.view.show_mood_page {
            self.moods_page(ctx);
        } else {
            if self.render.view.show_search_page {
                self.search_page(ctx);
            } else if self.render.view.show_file_picker {
                self.file_picker(ctx);
            } else {
                self.main_page(ctx);
            }
        }

        if self
            .render
            .view
            .show_about_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.about_viewport_startup(ctx);
        }
        if self
            .render
            .view
            .show_licenses_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.licenses_viewport_startup(ctx);
        }
        if self
            .render
            .view
            .show_help_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.help_viewport_startup(ctx);
        }
    }

    fn main_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                ui.add_space(1.0);
                ui.add(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        ui.menu_button("Urd", |ui: &mut Ui| {
                            if ui.button("Settings").clicked() {
                                if self.render.view.show_settings_viewport {
                                    self.render.view.show_settings_viewport = false;
                                    self.settings_backup = None;
                                } else {
                                    self.render.view.show_settings_viewport = true;
                                    self.settings_backup = Some(self.settings.clone());
                                }
                            }
                            if ui.button("About").clicked() {
                                self.render
                                    .view
                                    .show_about_viewport
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                            }
                            if ui.button("Licenses").clicked() {
                                self.render
                                    .view
                                    .show_licenses_viewport
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                            }
                            if ui.button("Help").clicked() {
                                self.render
                                    .view
                                    .show_help_viewport
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                            }
                        });
                        ui.menu_button("Journal", |ui: &mut Ui| {
                            if ui.button("Search").clicked() {
                                if self.render.view.show_search_page {
                                    self.render.view.show_search_page = false;
                                } else {
                                    self.render.view.show_search_page = true;
                                }
                            }
                            if ui.button("Important Days").clicked() {
                                if self.render.view.show_important_days_page {
                                    self.render.view.show_important_days_page = false;
                                } else {
                                    self.construct_important_day_entries();
                                    self.render.view.show_important_days_page = true;
                                    self.render.view.show_search_page = false;
                                    self.render.view.show_mood_page = false;
                                }
                            }
                            if ui.button("Moods").clicked() {
                                if self.render.view.show_mood_page {
                                    self.render.view.show_mood_page = false;
                                } else {
                                    self.construct_mood_entries();
                                    self.render.view.show_mood_page = true;
                                    self.render.view.show_search_page = false;
                                    self.render.view.show_important_days_page = false;
                                }
                            }
                            if ui.button("Export").clicked() {
                                if self.settings.custom_paths.export_directory != "" {
                                    let pos_err = self
                                        .journal
                                        .export(&self.settings.custom_paths.export_directory);
                                    if pos_err.is_err() {
                                        self.error = Error::new(
                                            pos_err.unwrap_err().to_string(),
                                            "Writing journal export to disk failed.".to_string(),
                                        );
                                    }
                                } else {
                                    self.settings.custom_paths.needed_path =
                                        Some(NeededPath::Export);
                                    self.render.view.show_file_picker = true;
                                }
                            }
                            ui.menu_button("Backup", |ui: &mut Ui| {
                                if ui.button("Create").clicked() {
                                    if self.settings.custom_paths.backup_directory != "" {
                                        // Backup already set up
                                        let pos_err = self.journal.create_backup(
                                            &self.settings,
                                            &self.settings.custom_paths.backup_directory,
                                        );
                                        if pos_err.is_err() {
                                            self.error = Error::new(
                                                pos_err.unwrap_err().to_string(),
                                                "Writing journal backup to disk failed."
                                                    .to_string(),
                                            );
                                        }
                                    } else {
                                        self.settings.custom_paths.needed_path =
                                            Some(NeededPath::Backup);
                                        self.render.view.show_file_picker = true;
                                    }
                                }
                                if ui.button("Restore").clicked() {
                                    self.settings.custom_paths.needed_path =
                                        Some(NeededPath::Restore);
                                    self.render.view.show_file_picker = true;
                                }
                            });
                        });
                        if self.render.view.show_important_days_page || self.render.view.show_mood_page {
                            if ui.button("Back to Home").clicked() {
                                self.render.view.show_important_days_page = false;
                                self.render.view.show_mood_page = false;
                            }
                        }
                        if self.settings.password.password != "" {
                            if ui.button("Lock Urd").clicked() {
                                self.settings.password.unlocked_with_password = false;
                            }
                        }
                        if self.render.view.show_file_picker {
                            if ui.button("Exit file picker").clicked() {
                                self.render.view.show_file_picker = false;
                            };
                        }
                    })
                    .response
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
        Box::new(|_cc| {
            Ok(Box::<UrdState>::new(UrdState::new(
                startup_state.settings,
                startup_state.journal,
                startup_state.error,
                startup_state.first_run,
            )))
        }),
    )
    .expect("E 01");
}
