use eframe::egui::{
    CentralPanel, Color32, Context, Grid, ScrollArea, Ui, Vec2, ViewportBuilder, ViewportId,
};

use super::UrdState;

const PADDING: f32 = 4.0;

impl UrdState {
    pub fn help_viewport_startup(&mut self, ctx: &Context) {
        if self
            .render
            .view
            .viewports
            .show_help_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let show_viewport_pointer = self.render.view.viewports.show_help_viewport.clone();
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("help_viewport"),
                ViewportBuilder::default()
                    .with_title("Help")
                    .with_inner_size([600.0, 800.0]),
                move |ctx, class| {
                    assert!(class == eframe::egui::ViewportClass::Deferred);
                    CentralPanel::default().show(ctx, |ui: &mut Ui| {
                        ui.vertical_centered_justified(|ui: &mut Ui| {
                            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    ui.group(|ui: &mut Ui| {
                                        ui.label("GUI");
                                        ui.collapsing("Tips and Tricks", |ui: &mut Ui| {
                                            ui.label("For more information check 'Tips and Tricks' inside the 'Features' section.");
                                            ui.separator();
                                            Grid::new("settings_tips").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                let _ = ui.button("Previous");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Go back to the previous tip.");
                                                });
                                                ui.end_row();
                                                let _ = ui.button("Next");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Go to the next tip.");
                                                });
                                                ui.end_row();
                                                let _ = ui.button("Dismiss");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Close the tips and tricks pop up.");
                                                });
                                                ui.end_row();
                                                let _ = ui.button("Don't show again");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Don't show the tips and tricks pop up again.");
                                                    ui.label("You can renable it in the settings.");
                                                });
                                                ui.end_row();
                                            });
                                        });
                                        ui.collapsing("Main menu", |ui: &mut Ui| {
                                            ui.label("The main menu is the row of buttons at the very top of Urd.");
                                            ui.separator();
                                            Grid::new("main_menu").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                
                                                let _ = ui.button("Urd");
                                                Grid::new("main_menu_urd").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("About");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the about page.");
                                                        ui.label("Check 'About page' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Exit");
                                                    ui.label("Exits Urd and saves all unsaved data.");
                                                    ui.end_row();

                                                    ui.label("Help");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the help page.");
                                                        ui.label("Check 'Help page' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Licenses");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the licenses page.");
                                                        ui.label("Check 'Licenses page' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Settings");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the settings page.");
                                                        ui.label("Check 'Settings page' for more information.");
                                                    });
                                                    ui.end_row();
                                                });
                                                ui.end_row();

                                                let _ = ui.button("Journal");
                                                Grid::new("main_menu_journal").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Search");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the search page.");
                                                        ui.label("Check 'Search page' or 'Search' under 'Features' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Important days");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the important days page.");
                                                        ui.label("Check 'Important days page' or 'Important days' under 'Features' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Moods");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the moods page.");
                                                        ui.label("Check 'Moods page' or 'Moods' under 'Features' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Export");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Export the entire journal.");
                                                        ui.label("Launches the export wizard if no export location is set.");
                                                        ui.label("Check 'Export' under 'Features' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Backup");
                                                    Grid::new("main_menu_journal_backup").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Create");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Creates a backup of the journal.");
                                                            ui.label("Launches the backup wizard if no backup location is set.");
                                                            ui.label("Check 'Backups' under 'Features' for more information.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("Restore");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Restores a journal from a backup of the journal.");
                                                            ui.label("Always launches the restore wizard.");
                                                            ui.label("Check 'Backups' under 'Features' for more information.")
                                                        });
                                                        ui.end_row();
                                                    });
                                                });

                                            });
                                        });
                                        ui.collapsing("Main page", |ui: &mut Ui| {
                                            Grid::new("main_page").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                ui.label("Side panel");
                                                Grid::new("side_panel").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Head");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The head of the side panel displays the current folder and allows you to navigate to other folders.");
                                                        ui.label("By clicking on the head you go to the parent folder.");
                                                        ui.separator();
                                                        Grid::new("side_panel_head").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                            ui.label("All years");
                                                            ui.vertical(|ui: &mut Ui| {
                                                                ui.label("This folder shows all years in the journal.");
                                                                ui.label("This is the root folder and is displayed by default.");
                                                                ui.label("Clicking on the head will do nothing.");
                                                            });
                                                            ui.end_row();

                                                            ui.label("Year");
                                                            ui.vertical(|ui: &mut Ui| {
                                                                ui.label("This is the folder for the current year.");
                                                                ui.label("On this level, you can see your aspirations for the year, if you have set any.");
                                                                ui.label("See 'Aspirations' under 'Concepts' for more information.");
                                                                ui.label("Clicking on the head will take you back to the root folder.");
                                                            });
                                                            ui.end_row();

                                                            ui.label("Month");
                                                            ui.vertical(|ui: &mut Ui| {
                                                                ui.label("This is the folder for the current month.");
                                                                ui.label("Clicking on the head will take you back to the year folder.");
                                                            });
                                                            ui.end_row();
                                                        });

                                                    });
                                                    ui.end_row();

                                                    ui.label("Body");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The body of the side panel displays the contents of the folder.");
                                                        ui.label("You can click on any entry shown to open it.");
                                                    });
                                                });
                                                ui.end_row();
                                            });

                                            // leave empthy, just for spacing
                                            //ui.label("");
                                            ui.separator();

                                            Grid::new("entry_editor").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                
                                                ui.label("Entry editor");
                                                Grid::new("entry_editor").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Text and entry menu");
                                                    Grid::new("main_menu_entry_editor_text_and_entry_menu").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Text Colour");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Changes the colour of the text.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("Font Size");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Changes the font size.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("Monospace");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Toggles monospace mode.");
                                                            ui.label("Monospace means that the text is displayed in a fixed width font.");
                                                            ui.label("Meaning that all letters have the same width.");
                                                        });
                                                        ui.end_row();

                                                        let _ = ui.button("Save entry");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Saves the entry.");
                                                        });
                                                        ui.end_row();

                                                        let _ = ui.button("Reset entry");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Resets the entry editor. Only the entry title is kept.");
                                                        });
                                                        ui.end_row();

                                                        let _ = ui.button("Todays entry");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Shortcut to open todays entry.");
                                                            ui.label("Use this if you don't want to restart Urd and a new day has begun.");
                                                        });
                                                        ui.end_row();

                                                        let _ = ui.button("Save entry and exit Urd");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Saves the entry and exits Urd.");
                                                            ui.label("More convenient than saving and navigating through the main menu.");
                                                        });
                                                    });
                                                    ui.end_row();

                                                    ui.label("Title");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("This is always the date of the entry and cannot be changed.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Mood");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Selects the mood of the entry.");
                                                        ui.label("You can choose a mood from the dropdown menu.");
                                                        ui.collapsing("Add mood", |ui: &mut Ui| {
                                                            ui.label("You can add a new mood by clicking the 'Add mood' button.");
                                                            ui.label("Now you set the name and Colour of the new mood.");
                                                            ui.label("It is important to remember that the name must be unique and cannot be changed or deleted later.");
                                                            ui.label("The colour can be changed later.");
                                                            ui.label("Confirm by clicking the 'Add mood' button again.");
                                                        });
                                                    });
                                                    ui.end_row();

                                                    ui.label("Important day");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("You can mark this entry as an important day with the checkbox.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Text field");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The text field is where you write the contents of the entry.");
                                                        ui.label("There are no length limits, and all formatting will be kept.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Metadata");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("All metadata is displayed below the 'Text field'.");
                                                        ui.label("You can click on any tag to search for entries with that tag.");
                                                        ui.label("It is recommended to keep a tags length within 42 characters.");
                                                        ui.label("For more information about metadata, check 'Tags' under 'Features'.");
                                                    });
                                                    ui.end_row();
                                                });
                                                ui.end_row();
                                            });
                                        });
                                        ui.collapsing("Settings page", |ui: &mut Ui| {
                                            ui.collapsing("Buttons", |ui: &mut Ui| {
                                                ui.label("The buttons on the settings page are always visible at the top.");
                                                Grid::new("settings_buttons").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Cancel");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.colored_label(Color32::RED, "Reverts all settings to the state they were in when the settings page was opened.");
                                                        ui.label("Does not save the settings and restores the previous settings.");
                                                        ui.label("Does not close the settings page.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Cancel and Close");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.colored_label(Color32::RED, "Reverts all settings to the state they were in when the settings page was opened.");
                                                        ui.label("Does not save the settings and restores the previous settings.");
                                                        ui.label("Closes the settings page.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Save");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Saves the settings.");
                                                        ui.label("Does not close the settings page.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Save and Close");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Saves the settings.");
                                                        ui.label("Closes the settings page.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Restore defaults");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.colored_label(Color32::RED, "Reverts all settings to the default state.");
                                                        ui.label("Restores the default settings.");
                                                        ui.label("Does not close the settings page.");
                                                    });
                                                    ui.end_row();
                                                });
                                            });
                                            ui.collapsing("Window settings", |ui: &mut Ui| {
                                                Grid::new("settings_window").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Window width");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The width of the main window.");
                                                        ui.label("Minimum: 100");
                                                        ui.label("Maximum: 3000");
                                                        ui.label("Overwrite min or max with the 'Overwrite window size' button.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Window height");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The height of the main window.");
                                                        ui.label("Minimum: 100");
                                                        ui.label("Maximum: 3000");
                                                        ui.label("Overwrite min or max with the 'Overwrite window size' button.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Side panel width");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The width of the side panel.");
                                                        ui.label("Minimum: 10");
                                                        ui.label("Maximum: 2000");
                                                        ui.label("Overwrite min or max with the 'Overwrite side panel width' button.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Overwrite window size");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Click the checkbox to enable the text fields.");
                                                        ui.label("You can enter any number you choose into the text fields.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Overwrite side panel width");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Click the checkbox to enable the text field.");
                                                        ui.label("You can enter any number you choose into the text field.");
                                                    });
                                                    ui.end_row();
                                                });
                                            });
                                            ui.collapsing("Font settings", |ui: &mut Ui| {
                                                Grid::new("settings_font").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Text colour");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The colour of the text.");
                                                        ui.label("Is automatically applied as soon as you choose a new colour.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Font size");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The size of the font.");
                                                        ui.label("Minimum: 2");
                                                        ui.label("Maximum: 100");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Monospace");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Toggle monospace on or off.");
                                                    });
                                                    ui.end_row();
                                                });
                                            });
                                            ui.collapsing("Security", |ui: &mut Ui| {
                                                Grid::new("settings_security").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Old password");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Enter your old password.");
                                                        ui.label("This text field is only enabled if a password has been set.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("New password");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Enter your new password.");
                                                        ui.label("This text field is always enabled.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Repeat new password");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Repeat your new password.");
                                                        ui.label("This text field is always enabled.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Set new password");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Save the new password.");
                                                        ui.label("You will be required to unlock the program with the new password.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Remove password");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("To enable, enter the old password. No new password is required.");
                                                        ui.label("Remove the password.");
                                                    });
                                                    ui.end_row();
                                                })
                                            });
                                            ui.collapsing("Date settings", |ui: &mut Ui| {
                                                Grid::new("settings_date").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Timezone");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The timezone used by Urd.");
                                                        ui.label("Urd does not support automatic daylight saving.");
                                                        ui.label("This means that you have to change the timezone manually when daylight saving is active.");
                                                    });
                                                    ui.end_row();
                                                })
                                            });
                                            ui.collapsing("File marker settings", |ui: &mut Ui| {
                                                ui.collapsing("Current file marker", |ui: &mut Ui| {
                                                    ui.label("The current file marker marks the current year and month.");
                                                    ui.separator();
                                                    Grid::new("settings_file_marker_currently").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Start");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The start of the file marker.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("End");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The end of the file marker.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                    })
                                                });
                                                ui.collapsing("Normally completed file marker", |ui: &mut Ui| {
                                                    ui.label("The normally completed file marker marks a year, month that has passed.");
                                                    ui.separator();
                                                    Grid::new("settings_file_marker_normally").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Start");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The start of the file marker.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("End");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The end of the file marker.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                    })
                                                });
                                                ui.collapsing("Perfectly completed file marker", |ui: &mut Ui| {
                                                    ui.label("The perfectly completed file marker marks a year where every month has at least one entry.");
                                                    ui.label("The perfectly completed file marker marks a month where every day has an entry.");
                                                    ui.separator();
                                                    Grid::new("settings_file_marker_perfectly").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Start");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The start of the file marker.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("End");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The end of the file marker.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                    })
                                                });
                                            });
                                            ui.collapsing("Backup settings", |ui: &mut Ui| {
                                                Grid::new("settings_backup").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Path");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The location where the backups are stored.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Automatic backup");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Automatically create a backup of the journal on every startup.");
                                                        ui.label("For more information check 'Backups' under 'Features'");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Launch backup wizard");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Launches the backup wizard to set a backup location.");
                                                    });
                                                    ui.end_row();
                                                })
                                            });
                                            ui.collapsing("Export settings", |ui: &mut Ui| {
                                                Grid::new("settings_export").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Path");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("The location where the export will be stored.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Launch export wizard");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Launches the export wizard to set an export location.");
                                                    });
                                                    ui.end_row();
                                                })
                                            });
                                            ui.collapsing("Mood settings", |ui: &mut Ui| {
                                                ui.collapsing("Create mood", |ui: &mut Ui| {
                                                    Grid::new("settings_mood_create").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Mood name");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The name of the mood.");
                                                            ui.label("The name must be unique and cannot be changed later.");
                                                            ui.label("You can use as many characters as you like.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("Mood colour");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The color of the mood.");
                                                            ui.label("Does not need to be unique and can be changed later.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("Add mood");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Adds the mood to the journal.");
                                                        });
                                                        ui.end_row();
                                                    })
                                                });
                                                ui.collapsing("All moods", |ui: &mut Ui| {
                                                    ui.label("Lists all moods in the journal.");
                                                    ui.label("You can change the colour of any mood here.");
                                                });
                                            });
                                            ui.collapsing("Aspirations settings", |ui: &mut Ui| {
                                                ui.label("By clicking on 'Show all years', you can edit aspirations for all years exsistant in the journal.");
                                                ui.label("By default only the current year is displayed.");
                                                ui.collapsing("Aspirations of a year", |ui: &mut Ui| {
                                                    Grid::new("settings_aspiration").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("Theme");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The theme of the year.");
                                                            ui.label("Should only be a word, maybe a few.");
                                                            ui.label("For more information check 'Aspirations' under 'Concepts'.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("Pledge");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The pledge of the year.");
                                                            ui.label("Should only be a short sentence.");
                                                            ui.label("For more information check 'Aspirations' under 'Concepts'.");
                                                        });
                                                        ui.end_row();
                                                        ui.label("Resolutions");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("The resolutions of the year.");
                                                            ui.label("Add as many as you like by using the 'Add resolution' button.");
                                                        });
                                                        ui.end_row();
                                                    })
                                                })
                                            });
                                            ui.collapsing("Tips and Tricks settings", |ui: &mut Ui| {
                                                Grid::new("settings_tips").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Show tips and tricks at startup");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Toggle showing tips and tricks at startup.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Show tips and tricks");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Launch the tips and tricks pop up.");
                                                    });
                                                    ui.end_row();
                                                })
                                            });
                                        });
                                        ui.collapsing("Search page", |ui: &mut Ui| {
                                            ui.collapsing("Search menu", |ui: &mut Ui| {
                                                Grid::new("search_menu").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Back");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Go back to home.");
                                                    });
                                                    ui.end_row();
                                                    ui.label("Clear");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Clears the search.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Search");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Searches for the query.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Search text field");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Enter the query to search for.");
                                                        ui.label("You can use tag prefixes, but they are not required.");
                                                        ui.label("Separate elements with commas.")
                                                    });
                                                    ui.end_row();
                                                })
                                            });
                                            ui.collapsing("Search results", |ui: &mut Ui| {
                                                ui.label("Displays the search results.");
                                                ui.label("If no results are found, nothing will be displayed.");
                                                ui.label("You can open any entry by clicking on it.");
                                            });
                                        });
                                        ui.collapsing("Moods page", |ui: &mut Ui| {
                                            ui.label("All moods are displayed at the top of the page.");
                                            ui.label("They are coloured according to their colour.");
                                            ui.label("All years are displayed below the moods. ");
                                            ui.label("Every day with an entry is displayed as a cube coloured in the mood colour.");
                                            ui.label("You can click on a cube to open the entry.");
                                        });
                                        ui.collapsing("Important days page", |ui: &mut Ui| {
                                            ui.label("All entries with the 'important' tag are displayed here.");
                                            ui.label("You can open any entry by clicking on it.");
                                        });
                                        ui.collapsing("File picker dialog", |ui: &mut Ui| {
                                            ui.label("Simply drag and drop the folder or file into the window.");
                                            ui.separator();
                                            Grid::new("file_picker").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                ui.label("Backup");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Needs a folder or directory to put the backup files into.");
                                                });
                                                ui.end_row();

                                                ui.label("Export");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Needs a folder or directory to put the export files into.");
                                                });
                                                ui.end_row();

                                                ui.label("Restore");
                                                ui.vertical(|ui: &mut Ui| {
                                                    ui.label("Needs a backup '.xff' file to restore the journal from.");
                                                });
                                                ui.end_row();
                                            })
                                        });
                                    });
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.label("Features");
                                    ui.collapsing("Password protection", |ui: &mut Ui| {
                                        ui.label("Urd can be protected with a password.");
                                        ui.label("This is not enabled by default and can be toggled in the settings.");
                                        ui.label("For more information about setting a password refer to the 'Security' section under 'GUI' -> 'Settings page'.");
                                        ui.separator();
                                        ui.label("The password protection is only designed to hide the journal contents from accidental exposure to a (technical illiterate) spouse, child or similar.");
                                        ui.separator();
                                        ui.label("It is important to note some technical details.");
                                        Grid::new("password_protection").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                            ui.label("1.");
                                            ui.label("The password is saved in plain text.");
                                            ui.end_row();

                                            ui.label("2.");
                                            ui.label("The password is not encrypted.");
                                            ui.end_row();

                                            ui.label("3.");
                                            ui.label("The password can be easily removed and the journal accessed without it.");
                                            ui.end_row();
                                        });
                                        ui.collapsing("Removing the password", |ui: &mut Ui| {
                                            ui.collapsing("With password", |ui: &mut Ui| {
                                                Grid::new("password_protection_removal").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("1.");
                                                    ui.label("Head to the settings page, and scroll to the 'Security' section.");
                                                    ui.end_row();

                                                    ui.label("2.");
                                                    ui.label("Enter the old password.");
                                                    ui.end_row();

                                                    ui.label("3.");
                                                    ui.label("Click 'Remove password'");
                                                    ui.end_row();
                                                })
                                            });
                                            ui.collapsing("Without password", |ui: &mut Ui| {
                                                ui.collapsing("Recommended way", |ui: &mut Ui| {
                                                    ui.colored_label(Color32::RED, "WARNING");
                                                    ui.label("This will revert all settings to their default values.");
                                                    ui.label("No journal data will be lost.");
                                                    ui.separator();
                                                    Grid::new("password_protection_removal_recommended").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("1.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Navigate to the 'urd_data' folder.");
                                                            ui.label("You can find it in the same folder as the urd executable.")
                                                        });
                                                        ui.end_row();

                                                        ui.label("2.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Delete the 'settings.xff' file inside the 'urd_data' folder.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("3.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Start 'Urd' again, the password has been removed.");
                                                            ui.label("All settings will be restored to their default values.");
                                                        });
                                                        ui.end_row();
                                                    });
                                                });
                                                ui.collapsing("Alternative way", |ui: &mut Ui| {
                                                    ui.colored_label(Color32::RED, "WARNING");
                                                    ui.label("This method requires some technical knowledge.");
                                                    ui.label("No data whatsoever will be lost.");
                                                    ui.separator();
                                                    Grid::new("password_protection_removal_alternative").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                        ui.label("1.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("You require a program to view binary files.");
                                                            ui.label("These can be found by searching for 'hex editor' or 'binary editor' online.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("2.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Navigate to the 'urd_data' folder.");
                                                            ui.label("You can find it in the same folder as the urd executable.")
                                                        });
                                                        ui.end_row();

                                                        ui.label("3.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Open the 'settings.xff' file inside the 'urd_data' folder with your chosen program.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("4.");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Look for the 'password' keyword in the ascii / utf8 representation of the file.");
                                                            ui.label("Most editors will have this enabled by default.");
                                                            ui.label("The text following the 'password' keyword is the password.");
                                                        })
                                                    })
                                                });
                                            });
                                        });
                                    });
                                    ui.collapsing("Backups", |ui: &mut Ui| {
                                    });
                                    ui.collapsing("Exporting", |ui: &mut Ui| {
                                    });
                                    ui.collapsing("Tips and Tricks", |ui: &mut Ui| {
                                        ui.label("For information on the 'GUI' elements check 'Tips and Tricks' under 'GUI'.");
                                        ui.separator();
                                        ui.label("Urd can display tips and tricks to help you get started.");
                                        ui.label("This is enabled by default and can be toggled in the settings.");
                                    });
                                    ui.collapsing("Search", |ui: &mut Ui| {
                                    });
                                    ui.collapsing("Moods", |ui: &mut Ui| {
                                    });
                                    ui.collapsing("Important days", |ui: &mut Ui| {
                                    });
                                    ui.collapsing("Tags", |ui: &mut Ui| {
                                        ui.label("It is recommended to keep a tags length within 42 characters.");
                                    });
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.label("Concepts");
                                    ui.collapsing("Journal", |ui: &mut Ui| {
                                    });
                                    ui.collapsing("Aspirations", |ui: &mut Ui| {
                                    })
                                });
                            });
                        });
                    });
                    // Close viewport
                    if ctx.input(|i| i.viewport().close_requested()) {
                        show_viewport_pointer.store(false, std::sync::atomic::Ordering::Relaxed);
                    }
                },
            );
        }
    }
}
