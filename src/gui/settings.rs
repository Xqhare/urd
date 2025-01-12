use eframe::egui::{
    Align, Color32, ComboBox, Context, Grid, ScrollArea, SidePanel, Sides, Slider, TextEdit, Ui,
};
use horae::TimeZone;
use nabu::{Object, XffValue};

use crate::{
    error::Error,
    moods::{default_moods, Mood},
    settings::{
        NeededPath, Settings, MAX_FONT_SIZE, MAX_SIDE_PANEL_WIDTH, MAX_WINDOW_SIZE, MIN_FONT_SIZE,
    },
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
                                            self.state_store.all_moods = Vec::new();
                                            self.settings.overwrite_window_size = false;
                                            self.settings.overwrite_side_panel_width = false;
                                            self.render.view.pages.show_settings_page = false;
                                        };
                                        but
                                    });
                                    ui.end_row();
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        let but = ui.button("Save");
                                        if but.clicked() {
                                            self.export_and_save_moods();
                                            let save_settings = self.settings.save();
                                            let save_journal = self.journal.save();
                                            if save_settings.is_err() {
                                                self.error = Error::new(save_settings.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                            } else if save_journal.is_err() {
                                                self.error = Error::new(save_journal.unwrap_err().to_string(), "Writing journal to disk failed.".to_string());
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
                                            self.export_and_save_moods();
                                            let save_settings = self.settings.save();
                                            let save_journal = self.journal.save();
                                            if save_settings.is_err() {
                                                self.error = Error::new(save_settings.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                            } else if save_journal.is_err() {
                                                self.error = Error::new(save_journal.unwrap_err().to_string(), "Writing journal to disk failed.".to_string());
                                            } else {
                                                self.settings.overwrite_window_size = false;
                                                self.render.view.pages.show_settings_page = false;
                                                self.settings.overwrite_side_panel_width = false;
                                                self.settings_backup = None;
                                                self.state_store.all_moods = Vec::new();
                                            }
                                        };
                                        but
                                    });
                                });
                                if ui.button("Restore defaults").clicked() {
                                    self.settings = Settings::default();
                                    self.settings_backup = Some(self.settings.clone());
                                    let save_settings = self.settings.save();
                                    let save_journal = self.journal.save();
                                    if save_settings.is_err() {
                                        self.error = Error::new(save_settings.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                    } else if save_journal.is_err() {
                                        self.error = Error::new(save_journal.unwrap_err().to_string(), "Writing journal to disk failed.".to_string());
                                    }
                                };
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Window settings");
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Window width: ");
                                }, |ui: &mut Ui| {
                                    ui.add(Slider::new(&mut self.settings.size.size[0], 100.0..=MAX_WINDOW_SIZE[0]));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Window height: ");
                                }, |ui: &mut Ui| {
                                    ui.add(Slider::new(&mut self.settings.size.size[1], 100.0..=MAX_WINDOW_SIZE[1]));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Side panel width: ");
                                }, |ui: &mut Ui| {
                                    ui.add(Slider::new(&mut self.settings.size.side_panel_width, 10.0..=MAX_SIDE_PANEL_WIDTH));
                                });
                                Grid::new("window_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.checkbox(&mut self.settings.overwrite_window_size, "Overwrite window Size");
                                    ui.add_enabled(self.settings.overwrite_window_size, |ui: &mut Ui| {
                                        ui.vertical_centered_justified(|ui: &mut Ui| {
                                            let width_txt = ui.add(TextEdit::singleline(&mut self.settings.overwrite_window_size_store[0]).horizontal_align(Align::Center));
                                            ui.spacing();
                                            ui.label("x");
                                            ui.spacing();
                                            let height_txt = ui.add(TextEdit::singleline(&mut self.settings.overwrite_window_size_store[1]).horizontal_align(Align::Center));
                                            if width_txt.changed() || height_txt.changed() {
                                                let overwrite_width = self.settings.overwrite_window_size_store[0].parse::<f32>();
                                                let overwrite_height = self.settings.overwrite_window_size_store[1].parse::<f32>();
                                                if overwrite_height.is_err() {
                                                    self.error = Error::new(format!("{} = {}", self.settings.overwrite_window_size_store[1], overwrite_height.unwrap_err()), "Invalid window size height input".to_string());
                                                    self.settings.overwrite_window_size_store[1] = "".to_string();
                                                } else if overwrite_width.is_err() {
                                                    self.error = Error::new(format!("{} = {}", self.settings.overwrite_window_size_store[0], overwrite_width.unwrap_err()), "Invalid window size width input".to_string());
                                                    self.settings.overwrite_window_size_store[0] = "".to_string();
                                                } else {
                                                    self.settings.size.size[0] = overwrite_width.unwrap();
                                                    self.settings.size.size[1] = overwrite_height.unwrap();
                                                }
                                            }
                                        }).response
                                    });
                                    ui.end_row();

                                    ui.checkbox(&mut self.settings.overwrite_side_panel_width, "Overwrite side panel width");
                                    ui.add_enabled(self.settings.overwrite_side_panel_width, |ui: &mut Ui| {
                                        ui.vertical_centered_justified(|ui: &mut Ui| {
                                            if ui.add(TextEdit::singleline(&mut self.settings.overwrite_side_panel_width_store).horizontal_align(Align::Center)).changed() {
                                                let overwrite_panel_width = self.settings.overwrite_side_panel_width_store.parse::<f32>();
                                                if overwrite_panel_width.is_err() {
                                                    self.error = Error::new(format!("{} = {}", self.settings.overwrite_side_panel_width_store, overwrite_panel_width.unwrap_err()), "Invalid side panel width input".to_string());
                                                    self.settings.overwrite_side_panel_width_store = self.settings.size.side_panel_width.to_string();
                                                } else {
                                                    self.settings.size.side_panel_width = overwrite_panel_width.unwrap();
                                                }
                                                println!("panel change triggered");
                                            };
                                        }).response
                                    });
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Font settings");
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Text colour: ");
                                }, |ui: &mut Ui| {
                                    if ui.color_edit_button_srgba(&mut self.settings.font.text_colour).changed() {
                                        let save = self.settings.save();
                                        if save.is_err() {
                                            self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                        }
                                    };
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Font size: ");
                                }, |ui: &mut Ui| {
                                    ui.add(Slider::new(&mut self.settings.font.size, MIN_FONT_SIZE..=MAX_FONT_SIZE));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Monospace: ");
                                }, |ui: &mut Ui| {
                                        if self.settings.font.monospace {
                                            ui.checkbox(&mut self.settings.font.monospace, "Enabled");
                                        } else {
                                            ui.checkbox(&mut self.settings.font.monospace, "Disabled");
                                        }
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                let pw_set = if self.settings.password.password == "" { false } else { true };
                                ui.label("Security");
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Old password: ");
                                }, |ui: &mut Ui| {
                                    ui.add_enabled(pw_set, |ui: &mut Ui| {
                                        ui.add(TextEdit::singleline(&mut self.settings.password.password_input).horizontal_align(Align::Center).password(true))
                                    });
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("New password: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.password.new_password_input[0]).horizontal_align(Align::Center).password(true));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Repeat new password: ");
                                }, |ui: &mut Ui| {
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
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Timezone: ");
                                }, |ui: &mut Ui| {
                                    ComboBox::from_label("").selected_text(self.settings.timezone.timezone.to_string()).show_ui(ui, |ui: &mut Ui| {
                                        for tz in self.settings.timezone.all_timezones_str.iter() {
                                            if ui.selectable_value(&mut self.settings.timezone.timezone, TimeZone::from(tz.clone()), tz.to_string()).clicked() {
                                                let save = self.settings.save();
                                                if save.is_err() {
                                                    self.error = Error::new(save.unwrap_err().to_string(), "Writing settings to disk failed.".to_string());
                                                }
                                            }
                                        }
                                    })
                                });
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("File marker Settings");
                                let year = horae::Utc::now().date().year.to_string();
                                ui.label("Current file marker").on_hover_text("Used for the current day / month / year.");
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Start: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_currently.start).horizontal_align(Align::Center));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("End: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_currently.end).horizontal_align(Align::Center));
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
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Start: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_normally.start).horizontal_align(Align::Center));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("End: ");
                                }, |ui: &mut Ui| {
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
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Start: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.gui.file_marker_perfectly.start).horizontal_align(Align::Center));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("End: ");
                                }, |ui: &mut Ui| {
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
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Path: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.custom_paths.backup_directory).horizontal_align(Align::Center));
                                });
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Automatic backup");
                                }, |ui: &mut Ui| {
                                    ui.checkbox(&mut self.settings.automatic_backups, "Every launch");
                                });
                                if ui.button("Launch backup wizard").clicked() {
                                    self.render.view.pages.show_file_picker_page = true;
                                    self.settings.custom_paths.needed_path = Some(NeededPath::Backup);
                                    self.render.view.pages.show_settings_page = false;
                                }
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Export settings");
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Path: ");
                                }, |ui: &mut Ui| {
                                    ui.add(TextEdit::singleline(&mut self.settings.custom_paths.export_directory).horizontal_align(Align::Center));
                                });
                                if ui.button("Launch export wizard").clicked() {
                                    self.render.view.pages.show_file_picker_page = true;
                                    self.settings.custom_paths.needed_path = Some(NeededPath::Export);
                                    self.render.view.pages.show_settings_page = false;
                                }
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Mood settings");

                                ui.group(|ui: &mut Ui| {
                                    Sides::new().show(ui, |ui: &mut Ui| {
                                        ui.label("Mood Name: ");
                                    }, |ui: &mut Ui| {
                                        ui.text_edit_singleline(&mut self.state_store.new_mood.name).on_hover_text("Enter the name of the new mood - This cannot be changed later!");
                                    });
                                    Sides::new().show(ui, |ui: &mut Ui| {
                                        ui.label("Mood Colour: ");
                                    }, |ui: &mut Ui| {
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
                                            self.render.view.ui_state.show_add_mood_field = false;
                                        }
                                    };
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.label("All moods");
                                    for mood in self.state_store.all_moods.iter_mut() {
                                        if mood.name != "" {
                                            Sides::new().show(ui, |ui: &mut Ui| {
                                                ui.label(&mood.name);
                                            }, |ui: &mut Ui| {
                                                ui.color_edit_button_srgba(&mut mood.colour);
                                                });
                                        }
                                    }
                                    if ui.button("Save").clicked() {
                                        self.export_and_save_moods();
                                    }
                                });
                                if !self.render.view.ui_state.show_destructive_action_confirmation {
                                    if ui.button("Restore default moods").clicked() {
                                        self.render.view.ui_state.show_destructive_action_confirmation = true;
                                                                            }
                                } else {
                                    ui.vertical_centered_justified(|ui: &mut Ui| {
                                        ui.heading("This action is destructive. If you have used ANY non default moods your journal will be unreadable!");
                                        ui.scope(|ui: &mut Ui| {
                                            ui.visuals_mut().override_text_color = Some(Color32::from_rgb(255, 0, 0));
                                            if ui.button("I understand, proceed").clicked() {
                                                self.journal.moods = default_moods();
                                                let save = self.journal.save();
                                                if save.is_err() {
                                                    self.error = Error::new(
                                                        save.unwrap_err().to_string(),
                                                        "Writing journal to disk failed.".to_string(),
                                                    );
                                                }
                                                self.render.view.ui_state.show_destructive_action_confirmation = false;
                                            }
                                        });
                                        if ui.button("Cancel").clicked() {
                                            self.render.view.ui_state.show_destructive_action_confirmation = false;
                                        }
                                    });
                                }
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Aspirations settings");
                                for entry in self.render.entities.aspirations.iter_mut() {
                                    ui.group(|ui: &mut Ui| {
                                        ui.label(format!("{}", entry.year));
                                        Sides::new().show(ui, |ui: &mut Ui| {
                                            ui.label("Theme: ");
                                        }, |ui: &mut Ui| {
                                            ui.add(TextEdit::singleline(&mut entry.edit_theme).horizontal_align(Align::Center).hint_text("Progress")).on_hover_text("Enter the theme of the year - One word only.");
                                        });
                                        Sides::new().show(ui, |ui: &mut Ui| {
                                            ui.label("Pledge: "); 
                                        }, |ui: &mut Ui| {
                                            ui.add(TextEdit::singleline(&mut entry.edit_pledge).horizontal_align(Align::Center).hint_text("Walk more")).on_hover_text("Enter the pledge of the year - One sentence max.");
                                        });
                                        ui.vertical_centered_justified(|ui: &mut Ui| {
                                            ui.label("Resolutions: ");
                                            for resolution in entry.edit_resolutions.iter_mut() {
                                                ui.add(TextEdit::singleline(resolution).horizontal_align(Align::Center).hint_text("Go to the gym")).on_hover_text("Enter one resolution per line.");
                                            }
                                        });
                                        if ui.button("Add resolution").clicked() {
                                            entry.edit_resolutions.push("".to_string());
                                        }
                                    });
                                    if ui.button("Save").clicked() {
                                        let xff_val = {
                                            let mut out = Object::new();
                                            out.insert("theme".to_string(), XffValue::from(entry.edit_theme.clone()));
                                            out.insert("pledge".to_string(), XffValue::from(entry.edit_pledge.clone()));
                                            out.insert("resolutions".to_string(), XffValue::from(entry.edit_resolutions.clone()));
                                            XffValue::from(out)
                                        };

                                        for year in self.journal.entries.iter_mut() {
                                            if year.get_folder().unwrap().name == entry.year {
                                                year.get_folder_mut().unwrap().aspirations = xff_val;
                                                break;
                                            }
                                        }
                                        let save = self.journal.save();
                                        if save.is_err() {
                                            self.error = Error::new(
                                                save.unwrap_err().to_string(),
                                                "Writing journal to disk failed.".to_string(),
                                            );
                                        }
                                    }
                                };
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Tips and Tricks settings");
                                Sides::new().show(ui, |ui: &mut Ui| {
                                    ui.label("Show tips and tricks at startup: ");
                                }, |ui: &mut Ui| {
                                        if self.settings.tips_and_tricks_at_startup {
                                            ui.checkbox(&mut self.settings.tips_and_tricks_at_startup, "Enabled");
                                        } else {
                                            
                                            ui.checkbox(&mut self.settings.tips_and_tricks_at_startup, "Disabled");
                                        }
                                });
                            }).response
                        });

                    });
                });
            });
        });
    }

    fn export_and_save_moods(&mut self) {
        let mut tmp = Object::new();
        for mood in self.state_store.all_moods.iter() {
            tmp.insert(mood.name.clone(), mood.colour.to_array().to_vec());
        }
        self.journal.moods = tmp;
        let save = self.journal.save();
        if save.is_err() {
            self.error = Error::new(
                save.unwrap_err().to_string(),
                "Writing journal to disk failed.".to_string(),
            );
        }
    }
}
