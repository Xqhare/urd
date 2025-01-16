
use eframe::egui::{Align, CentralPanel, Color32, ComboBox, Context, Grid, ProgressBar, Sides, Slider, TextEdit, TopBottomPanel, Ui};
use horae::TimeZone;
use nabu::{Object, XffValue};

use crate::{error::Error, paths::APP_DIR, settings::{MAX_FONT_SIZE, MIN_FONT_SIZE}};

use super::UrdState;

impl UrdState {
    pub fn welcome_page(&mut self, ctx: &Context) {
        if self.render.show_setup_wizard {
            self.setup_wizard(ctx);
        } else {
            self.welcome_screen(ctx);
        }
    }

    fn welcome_screen(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.add_space(ui.available_height() / 3.0);

            ui.add_sized([ui.available_width() / 1.33, 150.0], |ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.add_space(ui.available_width() / 3.0);
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.group(|ui: &mut Ui| {
                            ui.heading("Welcome to Urd");
                            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                            ui.separator();
                            ui.label("To get started, click the \"Setup wizard\" button below.");
                            ui.label("You can also skip the setup wizard and start using Urd right away.");
                            ui.label("Have fun!");
                            ui.separator();
                            ui.horizontal(|ui: &mut Ui| {
                                ui.add_space(ui.available_width() / 3.0);
                                if ui.button("Setup wizard").clicked() {
                                    self.render.show_setup_wizard = true;
                                };
                                if ui.button("Skip setup").clicked() {
                                    self.state_store.first_run = false;
                                    // Create basic state on disk
                                    if let Err(err) = self.settings.save() {
                                        self.error = Error::new(
                                            err.to_string(),
                                            "Writing settings to disk failed.".to_string(),
                                        );
                                    }
                                    if let Err(err) = self.journal.save() {
                                        self.error = Error::new(
                                            err.to_string(),
                                            "Writing journal to disk failed.".to_string(),
                                        );
                                    }
                                }
                                if ui.button("Exit").clicked() {
                                    // remove APP_DIR - Next startup should again be a first time
                                    // start up
                                    let _ = std::fs::remove_dir_all(APP_DIR);
                                    // exit process
                                    std::process::exit(0);
                                };
                            });
                        });
                    });
                    ui.add_space(ui.available_width() / 3.0);
                }).response
            });
        });
    }

    fn setup_wizard(&mut self, ctx: &Context) {
        self.setup_top_panel(ctx);
        self.setup_central_panel(ctx);
    }

    fn setup_central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.add_space(ui.available_height() / 3.0);

            ui.add_sized([ui.available_width() / 1.33, 150.0], |ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.add_space(ui.available_width() / 3.0);
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.group(|ui: &mut Ui| {
                            match self.state_store.wizard_setup_step {
                                0 => self.setup_step_0(ui),
                                1 => self.setup_step_1(ui),
                                2 => self.setup_step_2(ui),
                                3 => self.setup_step_3(ui),
                                4 => self.setup_step_4(ui),
                                _ => self.end_setup(ui),
                            }
                        });
                    });
                    ui.add_space(ui.available_width() / 3.0);
                }).response
            });
            if self.error.show_error {
                self.error_modal(ui);
            };
        });
    }

    fn setup_step_0(&mut self, ui: &mut Ui) {
        ui.heading("Text Editor");
        ui.group(|ui: &mut Ui| {
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
        });
        if ui.button("Back").clicked() {
            self.render.show_setup_wizard = false;
        }
        if ui.button("Next").clicked() {
            if let Err(err) = self.settings.save() {
                self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
            }
            self.state_store.wizard_setup_step = 1;
            self.state_store.setup_wizard_progress = 0.20;
        }
    }

    fn setup_step_1(&mut self, ui: &mut Ui) {
        ui.heading("Security");
        ui.group(|ui: &mut Ui| {
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
                let pw_set = if self.settings.password.password == "" { false } else { true };
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

                    if let Err(err) = self.settings.save() {
                        self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
                    }
                }
                self.state_store.wizard_setup_step = 2;
                self.state_store.setup_wizard_progress = 0.40;
            };
        });
        if ui.button("Previous").clicked() {
            self.state_store.wizard_setup_step = 0;
            self.state_store.setup_wizard_progress = 0.0;
        }
        if ui.button("Skip").clicked() {
            self.state_store.wizard_setup_step = 2;
            self.state_store.setup_wizard_progress = 0.40;
        }
    }

    fn setup_step_2(&mut self, ui: &mut Ui) {
        ui.heading("Date and time");
        ui.group(|ui: &mut Ui| {
            Sides::new().show(ui, |ui: &mut Ui| {
                ui.label("Timezone: ");
            }, |ui: &mut Ui| {
                ComboBox::from_label("").selected_text(self.settings.timezone.timezone.to_string()).show_ui(ui, |ui: &mut Ui| {
                    for tz in self.settings.timezone.all_timezones_str.iter() {
                        if ui.selectable_value(&mut self.settings.timezone.timezone, TimeZone::from(tz.clone()), tz.to_string()).clicked() {
                            if let Err(err) = self.settings.save() {
                                self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
                            }
                        }
                    }
                })
            });
        });
        if ui.button("Previous").clicked() {
            self.state_store.wizard_setup_step = 1;
            self.state_store.setup_wizard_progress = 0.20;
        }
        if ui.button("Next").clicked() {
            if let Err(err) = self.settings.save() {
                self.error = Error::new(err.to_string(), "Writing settings to disk failed.".to_string());
            }
            self.state_store.wizard_setup_step = 3;
            self.state_store.setup_wizard_progress = 0.60;
        }
    }

    fn setup_step_3(&mut self, ui: &mut Ui) {
        ui.heading("Aspirations settings");
        
        self.render.entities.aspirations = self.construct_aspirations();
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
            if ui.button("Previous").clicked() {
                self.state_store.wizard_setup_step = 2;
                self.state_store.setup_wizard_progress = 0.40;
            }
            if ui.button("Next").clicked() {
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
                
                if let Err(err) = self.journal.save() {
                    self.error = Error::new(err.to_string(), "Writing journal to disk failed.".to_string());
                }

                self.state_store.wizard_setup_step = 4;
                self.state_store.setup_wizard_progress = 0.80;
            }
        };
    }

    fn setup_step_4(&mut self, ui: &mut Ui) {
        // Stub for the future
        // TODO: change "Previous" button in end_setup() to go to '4, 0.80'
        self.state_store.wizard_setup_step = 5;
        self.state_store.setup_wizard_progress = 1.0;
    }

    fn setup_top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("setup_top_panel").show(ctx, |ui: &mut Ui| {
            ui.vertical_centered_justified(|ui: &mut Ui| {
                ui.heading("Setup wizard");
                ui.add(ProgressBar::new(self.state_store.setup_wizard_progress).fill(Color32::GREEN).desired_height(5.0));
            });
        });
    }

    fn end_setup(&mut self, ui: &mut Ui) {
        ui.heading("Final steps");
        ui.group(|ui: &mut Ui| {
            Grid::new("final_steps").num_columns(1).striped(true).show(ui, |ui: &mut Ui| {
                ui.scope(|ui: &mut Ui| {
                    ui.label("Add Urd to your start up applications to never miss a day in your journal.");
                    ui.add_space(ui.available_width());
                });
                ui.end_row();
                ui.label("Consider adding Urd to your task bar or creating a shortcut.");
            });
        });
        if ui.button("Previous").clicked() {
            self.state_store.wizard_setup_step = 3;
            self.state_store.setup_wizard_progress = 0.60;
        }
        if ui.button("Finish").clicked() {
            if let Err(err) = self.journal.save() {
                self.error = Error::new(
                    err.to_string(),
                    "Writing journal to disk failed.".to_string(),
                );
            }

            if let Err(err) = self.settings.save() {
                self.error = Error::new(
                    err.to_string(),
                    "Writing settings to disk failed.".to_string(),
                );
            }

            self.render.show_setup_wizard = false;
            self.state_store.first_run = false;
        }
    }
}
