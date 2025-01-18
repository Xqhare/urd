use eframe::egui::{CentralPanel, Context, Grid, Id, ScrollArea, Ui, Vec2, ViewportBuilder, ViewportId};

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
                                            ui.label("For more information on how this works, refer to 'Tips and Tricks' inside the 'Features' section.");
                                            ui.collapsing("Previous", |ui: &mut Ui| {
                                                ui.label("Click 'Previous' to go back to the previous tip.");
                                            });
                                            ui.collapsing("Next", |ui: &mut Ui| {
                                                ui.label("Click 'Next' to go back to the next tip.");
                                            });
                                            ui.collapsing("Dismiss", |ui: &mut Ui| {
                                                ui.label("Click 'Dismiss' to close the tips and tricks pop up.");
                                            });
                                            ui.collapsing("Don't show again", |ui: &mut Ui| {
                                                ui.label("Click 'Don't show again' to never have the pop up open again.");
                                                ui.label("You can renable it in the settings.");
                                            });
                                        });
                                        ui.collapsing("Main menu", |ui: &mut Ui| {
                                            ui.collapsing("Menu: Urd", |ui: &mut Ui| {
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
                                            });
                                            ui.collapsing("Menu: Journal", |ui: &mut Ui| {
                                                Grid::new("main_menu_journal").num_columns(2).striped(true).spacing(Vec2::new(ui.spacing().item_spacing.x + PADDING, ui.spacing().item_spacing.y * PADDING)).show(ui, |ui: &mut Ui| {
                                                    ui.label("Search");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the search page.");
                                                        ui.label("Check 'Search page' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Important days");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the important days page.");
                                                        ui.label("Check 'Important days page' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Moods");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Opens the moods page.");
                                                        ui.label("Check 'Moods page' for more information.");
                                                    });
                                                    ui.end_row();

                                                    ui.label("Export");
                                                    ui.vertical(|ui: &mut Ui| {
                                                        ui.label("Export the entire journal.");
                                                        ui.label("Launches the export wizard if no export location is set.");
                                                        ui.label("Check 'Export' under 'Features' for more information.");
                                                    });
                                                    ui.end_row();
                                                });
                                                ui.collapsing("Backup", |ui: &mut Ui| {
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
                                                            ui.label("Check 'Backups' under 'Features' for more information.")
                                                        });
                                                        ui.end_row();
                                                    });
                                                });
                                            });
                                        });
                                        ui.collapsing("Main page", |ui: &mut Ui| {
                                            
                                            ui.collapsing("Side panel", |ui: &mut Ui| {
                                                ui.collapsing("Head", |ui: &mut Ui| {
                                                    ui.label("The head of the side panel displays the current folder and allows you to navigate to other folders.");
                                                    ui.label("By clicking on the head you go to the parent folder.");
                                                    ui.collapsing("All years", |ui: &mut Ui| {
                                                        ui.label("This is the root folder and is displayed by default.");
                                                        ui.label("Clicking on the head will do nothing.");
                                                    });
                                                    ui.collapsing("Year", |ui: &mut Ui| {
                                                        ui.label("On this level, you can see your aspirations for the year, if you have set any.");
                                                        ui.label("Clicking on the head will take you back to the root folder.");
                                                    });
                                                    ui.collapsing("Month", |ui: &mut Ui| {
                                                        ui.label("Clicking on the head will take you back to the year folder.");
                                                    });
                                                });
                                                ui.collapsing("Body", |ui: &mut Ui| {
                                                    ui.label("The body of the side panel displays the contents of the folder.");
                                                    ui.label("You can click on any entry shown to open it.");
                                                });
                                            });
                                            ui.collapsing("Entry editor", |ui: &mut Ui| {
                                                ui.collapsing("Text and entry menu", |ui: &mut Ui| {
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

                                                        ui.label("Save entry");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Saves the entry.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("Reset entry");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Resets the entry editor. Only the entry title is kept.");
                                                        });
                                                        ui.end_row();

                                                        ui.label("Todays entry");
                                                        ui.vertical(|ui: &mut Ui| {
                                                            ui.label("Shortcut to open todays entry.");
                                                            ui.label("Use this if you don't want to restart Urd and a new day has begun.");
                                                        });
                                                        ui.end_row();
                                                    })
                                                });
                                                ui.collapsing("Title", |ui: &mut Ui| {
                                                    ui.label("The title of the entry.");
                                                    ui.label("This is always the date of the entry and cannot be changed.");
                                                });
                                                ui.collapsing("Mood and important day", |ui: &mut Ui| {
                                                    ui.collapsing("Mood", |ui: &mut Ui| {
                                                        ui.label("You can choose a mood from the dropdown menu.");
                                                        ui.collapsing("Add mood", |ui: &mut Ui| {
                                                            ui.label("You can add a new mood by clicking the 'Add mood' button.");
                                                            ui.label("Now you set the name and Colour of the new mood.");
                                                            ui.label("It is important to remember that the name must be unique and cannot be changed or deleted later.");
                                                            ui.label("The colour can be changed later.");
                                                            ui.label("Confirm by clicking the 'Add mood' button again.");
                                                        });
                                                    });
                                                    ui.collapsing("Important day", |ui: &mut Ui| {
                                                        ui.label("You can mark this entry as an important day with the checkbox.");
                                                    });
                                                });
                                                ui.collapsing("Text field", |ui: &mut Ui| {
                                                    ui.label("The text field is where you write the contents of the entry.");
                                                    ui.label("There are now length limits, and all formatting will be kept.");
                                                });
                                                ui.collapsing("Metadata", |ui: &mut Ui| {
                                                    ui.label("All metadata is displayed below the 'Text field'.");
                                                    ui.label("You can click on any tag to search for entries with that tag.");
                                                });
                                            });
                                        });
                                        ui.collapsing("Settings page", |ui: &mut Ui| {
                                            
                                        });
                                        ui.collapsing("Search page", |ui: &mut Ui| {
                                            
                                        });
                                        ui.collapsing("Moods page", |ui: &mut Ui| {
                                            
                                        });
                                        ui.collapsing("Important days page", |ui: &mut Ui| {
                                            
                                        });
                                        ui.collapsing("File picker dialog", |ui: &mut Ui| {
                                            
                                        })
                                    });
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.label("Features");
                                    ui.collapsing("Password protection", |ui: &mut Ui| {
                                        
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
                                        
                                    });
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.label("Concepts");
                                    ui.collapsing("Journal", |ui: &mut Ui| {
                                        
                                    });
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
