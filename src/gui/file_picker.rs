use eframe::{
    egui::{CentralPanel, Ui},
    *,
};

use crate::{error::Error, settings::NeededPath};

use super::UrdState;

impl UrdState {
    pub fn file_picker(&mut self, ctx: &egui::Context) {
        debug_assert!(self.settings.custom_paths.needed_path.is_some());
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.vertical_centered_justified(|ui: &mut Ui| {
                if let Some(needed_path) = &self.settings.custom_paths.needed_path {
                    match needed_path {
                        NeededPath::Backup => {
                            ui.heading("Backup wizard");
                            ui.separator();
                            ui.label("Provide a backup path by dragging-and-dropping the directory into the window!");
                            ui.label("Automatic backups: ");
                            ui.checkbox(&mut self.settings.automatic_backups, "Every launch");
                            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                                let dropped_file = ctx.input(|i| {
                                    // The array of files is only larger than 1 if several files are
                                    // dropped in at once.
                                    i.raw.dropped_files[0].path.clone().expect("File without path").clone()
                                });
                                if dropped_file.is_dir() {
                                    self.settings.custom_paths.backup_directory = dropped_file.to_str().expect("Path with invalid utf8").to_string();
                                } else {
                                    self.error = Error::new("Provided a file, not a directory!".to_string(), "Invalid path".to_string());
                                }
                            }
                            if self.settings.custom_paths.backup_directory != "" {
                                ui.separator();
                                ui.label("Backup path: ");
                                ui.label(self.settings.custom_paths.backup_directory.clone());
                                ui.separator();
                                if ui.button("Done").on_hover_text("Save, create backup and close file picker").clicked() {
                                    self.render.view.pages.show_file_picker_page = false;
                                    if let Err(err) = self.settings.save() {
                                        self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
                                    }
                                    if let Err(err) = self.journal.create_backup(&self.settings, &self.settings.custom_paths.backup_directory) {
                                        self.error = Error::new(err.to_string(), "Creating backup failed.".to_string());
                                    }
                                }
                            }
                        },
                        NeededPath::Export => {
                            ui.heading("Export wizard");
                            ui.separator();
                            ui.label("Provide an export path by dragging-and-dropping the directory into the window!");
                            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                                let dropped_file = ctx.input(|i| {
                                    // The array of files is only larger than 1 if several files are
                                    // dropped in at once.
                                    i.raw.dropped_files[0].path.clone().expect("File without path").clone()
                                });
                                if dropped_file.is_dir() {
                                    self.settings.custom_paths.export_directory = dropped_file.to_str().expect("Path with invalid utf8").to_string();
                                } else {
                                    self.error = Error::new("Provided a file, not a directory!".to_string(), "Invalid path".to_string());
                                }
                            }
                            if self.settings.custom_paths.export_directory != "" {
                                ui.separator();
                                ui.label("Export path: ");
                                ui.label(self.settings.custom_paths.export_directory.clone());
                                ui.separator();
                                if ui.button("Done").on_hover_text("Save, export and close file picker").clicked() {
                                    self.render.view.pages.show_file_picker_page = false;
                                    if let Err(err) = self.settings.save() {
                                        self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
                                    }
                                    if let Err(err) = self.journal.export(&self.settings.custom_paths.export_directory) {
                                        self.error = Error::new(err.to_string(), "Creating backup failed.".to_string());
                                    }
                                }
                            }
                        },
                        NeededPath::Restore => {
                            ui.heading("Restore wizard");
                            ui.separator();
                            ui.label("Provide a backup file by dragging-and-dropping the file into the window!");
                            if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
                                let dropped_file = ctx.input(|i| {
                                    // The array of files is only larger than 1 if several files are
                                    // dropped in at once.
                                    i.raw.dropped_files[0].path.clone().expect("File without path").clone()
                                });
                                if dropped_file.is_file() {
                                    self.settings.custom_paths.restore_file = dropped_file.to_str().expect("Path with invalid utf8").to_string();
                                } else {
                                    self.error = Error::new("Provided a directory, not a file!".to_string(), "Invalid path".to_string());
                                }
                            }
                            if self.settings.custom_paths.restore_file != "" {
                                ui.separator();
                                ui.label("Restore file: ");
                                ui.label(self.settings.custom_paths.restore_file.clone());
                                ui.separator();
                                if ui.button("Restore from file").on_hover_text("Try to restore from file and close file picker").clicked() {
                                    self.render.view.pages.show_file_picker_page = false;
                                    if let Err(err) = self.settings.save() {
                                        self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
                                    }
                                    if let Err(err) = self.journal.restore_backup(&self.settings, &self.settings.custom_paths.restore_file) {
                                        self.error = Error::new(err.to_string(), "Restoring from backup failed.".to_string());
                                    }
                                }
                            }
                        },
                    }
                }
            });
        });
    }
}
