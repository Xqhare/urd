
use eframe::egui::{Align, ComboBox, Context, Grid, ScrollArea, SidePanel, Slider, TextEdit, Ui, Widget};
use horae::TimeZone;

use crate::{error::Error, settings::{self, Settings, MAX_FONT_SIZE, MAX_SIDE_PANEL_WIDTH, MAX_WINDOW_SIZE, MIN_FONT_SIZE}};

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
                                            self.settings.show_settings_viewport = false;
                                        };
                                        but
                                    });
                                    ui.end_row();
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        let but = ui.button("Save");
                                        if but.clicked() {
                                            let save = self.settings.save();
                                            if save.is_err() {
                                                self.error = Error::new(save.unwrap_err().to_string());
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
                                                self.error = Error::new(save.unwrap_err().to_string());
                                            } else {
                                                self.settings.overwrite_window_size = false;
                                                self.settings.show_settings_viewport = false;
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
                                        self.error = Error::new(save.unwrap_err().to_string());
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
                                        let overwrite_width = self.settings.overwrite_window_size_store[0].parse();
                                        let overwrite_height = self.settings.overwrite_window_size_store[1].parse();
                                        if overwrite_width.is_err() || overwrite_height.is_err() {
                                            self.error = Error::new("Invalid window size - Not a number".to_string());
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
                                        let overwrite_panel_width = self.settings.overwrite_side_panel_width_store.parse();
                                        if overwrite_panel_width.is_err() {
                                            self.error = Error::new("Invalid side panel size - Not a number".to_string());
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
                                            self.error = Error::new(save.unwrap_err().to_string());
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
                                                self.error = Error::new("Incorrect old password".to_string());
                                            }
                                        } else {
                                            self.settings.password.password = self.settings.password.new_password_input[0].to_string();
                                            set_pw_is_okay = true;
                                        }
                                    } else {
                                        self.error = Error::new("New password entries do not match".to_string());
                                    }

                                    if set_pw_is_okay {
                                        self.settings.password.password_input = String::new();
                                        self.settings.password.new_password_input[0] = String::new();
                                        self.settings.password.new_password_input[1] = String::new();

                                        let save = self.settings.save();
                                        if save.is_err() {
                                            self.error = Error::new(save.unwrap_err().to_string());
                                        }
                                    }
                                };
                            }).response
                        });

                        ui.add(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Date Settings");
                                Grid::new("date_settings").num_columns(2).show(ui, |ui: &mut Ui| {
                                    ui.scope(|ui: &mut Ui| {
                                        ui.label("Timezone: ");
                                        ui.add_space(250.0 / 3.75);
                                    });
                                    ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                                        ComboBox::from_label("").selected_text(self.settings.timezone.timezone.to_string()).show_ui(ui, |ui: &mut Ui| {
                                            for tz in self.settings.timezone.all_timezones_str.iter() {
                                                if ui.selectable_value(&mut self.settings.timezone.timezone, TimeZone::from(tz.clone()), tz.to_string()).clicked() {
                                                    let save = self.settings.save();
                                                    if save.is_err() {
                                                        self.error = Error::new(save.unwrap_err().to_string());
                                                    }
                                                }
                                            }
                                        }).response
                                    });
                                    ui.end_row();
                                });
                            }).response
                        });

                    });
                })
            })
        });
    }
}
