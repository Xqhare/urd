use eframe::egui::{Align, ComboBox, Context, Grid, ScrollArea, SidePanel, Slider, TextEdit, Ui};
use horae::TimeZone;

use crate::{
    error::Error, moods::Mood, settings::{
        NeededPath, Settings, MAX_FONT_SIZE, MAX_SIDE_PANEL_WIDTH, MAX_WINDOW_SIZE, MIN_FONT_SIZE,
    }
};

use super::UrdState;

impl UrdState {
    pub fn settings_viewport_startup(&mut self, ctx: &Context) {
        SidePanel::left("settings").default_width(self.settings.size.side_panel_width).show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.group(|ui: &mut Ui| {
                    ui.vertical_centered_justified(|ui: &mut Ui| {
                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                Grid::new("buttons").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        let but = ui.button("Cancel");
                                        if but.clicked() {
                                            self.settings = self.settings_backup.clone().unwrap();
                                        }
                                        but
                                    });
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        let but = ui.button("Cancel and Close");
                                        if but.clicked() {
                                            self.settings = self.settings_backup.clone().unwrap();
                                            self.settings.overwrite_window_size = false;
                                            self.settings.overwrite_side_panel_width = false;
                                            self.render.view.show_settings_viewport = false;
                                        };
                                        but
                                    });
                                    ui.end_row();
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        let but = ui.button("Save");
                                        if but.clicked() {
                                            let save = self.settings.save();
                                            if save.is_err() {
                                                self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                            } else {
                                                self.settings.overwrite_window_size = false;
                                                self.settings.overwrite_side_panel_width = false;
                                                self.settings_backup = Some(self.settings.clone());
                                            }
                                        };
                                        but
                                    });
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        let but = ui.button("Save and Close");
                                        if but.clicked() {
                                            let save = self.settings.save();
                                            if save.is_err() {
                                                self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                            } else {
                                                self.settings.overwrite_window_size = false;
                                                self.render.view.show_settings_viewport = false;
                                                self.settings.overwrite_side_panel_width = false;
                                                self.settings_backup = None;
                                            }
                                        };
                                        but
                                    });
                                });
                                if ui.button("Restore defaults").clicked() {
                                    self.settings = Settings::default();
                                    self.settings_backup = Some(self.settings.clone());
                                    let save = self.settings.save();
                                    if save.is_err() {
                                        self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                    }
                                };
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Window settings");
                                Grid::new("window_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Window width: ");
                                    ui.add(Slider::new(&mut self.settings.size.size[0], 100.0..=MAX_WINDOW_SIZE[0]));
                                    ui.end_row();

                                    ui.label("Window height: ");
                                    ui.add(Slider::new(&mut self.settings.size.size[1], 100.0..=MAX_WINDOW_SIZE[1]));
                                    ui.end_row();

                                    ui.checkbox(&mut self.settings.overwrite_window_size, "Overwrite window Size");
                                    ui.add_enabled(self.settings.overwrite_window_size, |ui: &mut Ui| {
                                        ui.vertical_centered_justified(|ui: &mut Ui| {
                                            ui.add(TextEdit::singleline(&mut self.settings.overwrite_window_size_store[0]).horizontal_align(Align::Center));
                                            ui.spacing();
                                            ui.label("x");
                                            ui.spacing();
                                            ui.add(TextEdit::singleline(&mut self.settings.overwrite_window_size_store[1]).horizontal_align(Align::Center));
                                        }).response
                                    });
                                    if self.settings.overwrite_window_size {
                                        let overwrite_width = self.settings.overwrite_window_size_store[0].parse::<f32>();
                                        let overwrite_height = self.settings.overwrite_window_size_store[1].parse::<f32>();
                                        if overwrite_height.is_err() {
                                            self.error = Error::new(format!("{} = {}", self.settings.overwrite_window_size_store[1], overwrite_height.unwrap_err()), "Invalid window size height input".to_string());
                                            self.settings.overwrite_window_size_store[1] = self.settings.overwrite_window_size_store[0].clone();
                                            return;
                                        } else if overwrite_width.is_err() {
                                            self.error = Error::new(format!("{} = {}", self.settings.overwrite_window_size_store[0], overwrite_width.unwrap_err()), "Invalid window size width input".to_string());
                                            self.settings.overwrite_window_size_store[0] = self.settings.overwrite_window_size_store[1].clone();
                                            return;
                                        } else {
                                            self.settings.size.size[0] = overwrite_width.unwrap();
                                            self.settings.size.size[1] = overwrite_height.unwrap();
                                        }
                                    }
                                    ui.end_row();

                                    ui.label("Side panel width: ");
                                    ui.add(Slider::new(&mut self.settings.size.side_panel_width, 10.0..=MAX_SIDE_PANEL_WIDTH));
                                    ui.end_row();

                                    ui.checkbox(&mut self.settings.overwrite_side_panel_width, "Overwrite side panel width");
                                    ui.add_enabled(self.settings.overwrite_side_panel_width, |ui: &mut Ui| {
                                        ui.vertical_centered_justified(|ui: &mut Ui| {
                                            ui.add(TextEdit::singleline(&mut self.settings.overwrite_side_panel_width_store).horizontal_align(Align::Center));
                                        }).response
                                    });
                                    if self.settings.overwrite_side_panel_width {
                                        let overwrite_panel_width = self.settings.overwrite_side_panel_width_store.parse::<f32>();
                                        if overwrite_panel_width.is_err() {
                                            self.error = Error::new(format!("{} = {}", self.settings.overwrite_side_panel_width_store, overwrite_panel_width.unwrap_err()), "Invalid side panel width input".to_string());
                                            self.settings.overwrite_side_panel_width_store = self.settings.size.side_panel_width.to_string();
                                            return;
                                        } else {
                                            self.settings.size.side_panel_width = overwrite_panel_width.unwrap();
                                        }
                                    }
                                    ui.end_row();
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Font settings");
                                Grid::new("font_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Text colour: ");
                                    ui.scope(|ui: &mut Ui| {
                                        ui.add_space(250.0 / 1.2);
                                        if ui.color_edit_button_srgba(&mut self.settings.font.text_colour).changed() {
                                            let save = self.settings.save();
                                            if save.is_err() {
                                                self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                            }
                                        };
                                    });
                                    ui.end_row();

                                    ui.label("Font size: ");
                                    ui.scope(|ui: &mut Ui| {
                                        ui.add_space(250.0 / 2.5);
                                        ui.add(Slider::new(&mut self.settings.font.size, MIN_FONT_SIZE..=MAX_FONT_SIZE));
                                    });
                                    ui.end_row();

                                    ui.label("Monospace: ");
                                    ui.scope(|ui: &mut Ui| {
                                        ui.add_space(250.0 / 1.2);
                                        ui.checkbox(&mut self.settings.font.monospace, "");
                                    });
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                let pw_set = if self.settings.password.password == "" { false } else { true };
                                ui.label("Security");
                                Grid::new("pw").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Old password: ");
                                    ui.add_enabled(pw_set, |ui: &mut Ui| {
                                        ui.add(TextEdit::singleline(&mut self.settings.password.password_input).horizontal_align(Align::Center).password(true))
                                    });
                                    ui.end_row();

                                    ui.label("New password: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.password.new_password_input[0]).horizontal_align(Align::Center).password(true));
                                    ui.end_row();

                                    ui.label("Repeat new password: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.password.new_password_input[1]).horizontal_align(Align::Center).password(true));
                                });
                                if ui.button("Set new password").clicked() {
                                    let mut set_pw_is_okay = false;
                                    if self.settings.password.new_password_input[0] == self.settings.password.new_password_input[1] {
                                        if pw_set {
                                            if self.settings.password.password == self.settings.password.password_input {
                                                self.settings.password.password = self.settings.password.new_password_input[0].to_string();
                                                set_pw_is_okay = true;
                                            } else {
                                                self.error = Error::new("Incorrect old password".to_string(), "Setting new password failed.".to_string());
                                            }
                                        } else {
                                            self.settings.password.password = self.settings.password.new_password_input[0].to_string();
                                            set_pw_is_okay = true;
                                        }
                                    } else {
                                        self.error = Error::new("New password entries do not match".to_string(), "Setting new password failed.".to_string());
                                    }

                                    if set_pw_is_okay {
                                        self.settings.password.password_input = String::new();
                                        self.settings.password.new_password_input[0] = String::new();
                                        self.settings.password.new_password_input[1] = String::new();

                                        let save = self.settings.save();
                                        if save.is_err() {
                                            self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                        }
                                    }
                                };
                                let remove_pw_allowed = if self.settings.password.password_input == self.settings.password.password && pw_set { true } else { false };
                                ui.add_enabled(remove_pw_allowed, |ui: &mut Ui| {
                                    let but = ui.button("Remove password");
                                    if but.clicked() {
                                        self.settings.password.password = "".to_string();
                                        self.settings.password.password_input = "".to_string();
                                        let save = self.settings.save();
                                        if save.is_err() {
                                            self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                        }
                                    }
                                    but
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Date settings");
                                Grid::new("date_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.scope(|ui: &mut Ui| {
                                        ui.label("Timezone: ");
                                        // justified with security
                                        ui.add_space(250.0 / 4.25);
                                    });
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        ComboBox::from_label("").selected_text(self.settings.timezone.timezone.to_string()).show_ui(ui, |ui: &mut Ui| {
                                            for tz in self.settings.timezone.all_timezones_str.iter() {
                                                if ui.selectable_value(&mut self.settings.timezone.timezone, TimeZone::from(tz.clone()), tz.to_string()).clicked() {
                                                    let save = self.settings.save();
                                                    if save.is_err() {
                                                        self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                                    }
                                                }
                                            }
                                        }).response
                                    });
                                    ui.end_row();
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("File marker Settings");
                                let year = horae::Utc::now().date().year.to_string();
                                ui.label("Current file marker").on_hover_text("Used for the current day / month / year.");
                                Grid::new("current_file_marker").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Start: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_currently.start).horizontal_align(Align::Center));
                                    ui.end_row();
                                    ui.label("End: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_currently.end).horizontal_align(Align::Center));
                                    ui.end_row();
                                });
                                ui.vertical_centered_justified(|ui: &mut Ui| {
                                    ui.group(|ui: &mut Ui| {
                                        ui.label("Example: ");
                                        let display_txt = format!("{} {} {}", self.settings.gui.file_marker_currently.start, year, self.settings.gui.file_marker_currently.end);
                                        ui.label(display_txt);
                                    });
                                });
                                ui.separator();
                                ui.label("Completed normally file marker").on_hover_text("Used if a month / year has passed. Marks 'Completed' years or months.");
                                Grid::new("completed_file_marker").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Start: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_normally.start).horizontal_align(Align::Center));
                                    ui.end_row();
                                    ui.label("End: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_normally.end).horizontal_align(Align::Center));
                                });
                                ui.vertical_centered_justified(|ui: &mut Ui| {
                                    ui.group(|ui: &mut Ui| {
                                        ui.label("Example: ");
                                        let display_txt = format!("{} {} {}", self.settings.gui.file_marker_normally.start, year, self.settings.gui.file_marker_normally.end);
                                        ui.label(display_txt);
                                    });
                                });
                                ui.separator();
                                ui.label("Perfectly completed file marker").on_hover_text("Used if a journal entry was made on every day of the month or year.");
                                Grid::new("perfectly_completed_file_marker").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Start: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_perfectly.start).horizontal_align(Align::Center));
                                    ui.end_row();
                                    ui.label("End: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_perfectly.end).horizontal_align(Align::Center));
                                });
                                ui.vertical_centered_justified(|ui: &mut Ui| {
                                    ui.group(|ui: &mut Ui| {
                                        ui.label("Example: ");
                                        let display_txt = format!("{} {} {}", self.settings.gui.file_marker_perfectly.start, year, self.settings.gui.file_marker_perfectly.end);
                                        ui.label(display_txt);
                                    });
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Backup settings");
                                Grid::new("backup_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Path: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.custom_paths.backup_directory).horizontal_align(Align::Center));
                                    ui.end_row();
                                    ui.label("Automatic backup");
                                    ui.checkbox(&mut self.settings.automatic_backups, "Every launch");
                                    ui.end_row();
                                });
                                if ui.button("Launch backup wizard").clicked() {
                                    self.render.view.show_file_picker = true;
                                    self.settings.custom_paths.needed_path = Some(NeededPath::Backup);
                                    self.render.view.show_settings_viewport = false;
                                }
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Export settings");
                                Grid::new("export_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Path: ");
                                    ui.add(TextEdit::singleline(&mut self.settings.custom_paths.export_directory).horizontal_align(Align::Center));
                                    ui.end_row();
                                });
                                if ui.button("Launch export wizard").clicked() {
                                    self.render.view.show_file_picker = true;
                                    self.settings.custom_paths.needed_path = Some(NeededPath::Export);
                                    self.render.view.show_settings_viewport = false;
                                }
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Mood settings");

                                Grid::new("mood_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.label("Mood Name: ");
                                    ui.text_edit_singleline(&mut self.state_store.new_mood.name).on_hover_text("Enter the name of the new mood");

                                    ui.end_row();

                                    ui.label("Mood Colour: ");
                                    ui.color_edit_button_srgba(&mut self.state_store.new_mood.colour).on_hover_text("Choose the colour of the new mood");
                                });
                                
                                if ui.button("Add mood").clicked() {
                                    if self.journal.moods.contains_key(&self.state_store.new_mood.name) {
                                        self.error = Error::new(
                                            "Mood already exists.".to_string(),
                                            "Please choose a different name.".to_string(),
                                        );
                                    } else {
                                        self.journal.moods.insert(self.state_store.new_mood.name.clone(), self.state_store.new_mood.colour.to_array().to_vec());
                                        let save = self.journal.save();
                                        if save.is_err() {
                                            self.error = Error::new(
                                                save.unwrap_err().to_string(),
                                                "Writing journal to disk failed.".to_string(),
                                            );
                                        }
                                        // Reset
                                        self.state_store.new_mood = Mood::default();
                                        self.render.show_add_mood_field = false;
                                    }
                                };
                            }).response
                        });

                    });
                })
            })
        });
    }
}
