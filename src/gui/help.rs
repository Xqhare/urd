use eframe::egui::{CentralPanel, Context, Id, ScrollArea, Ui, ViewportBuilder, ViewportId};

use super::UrdState;

impl UrdState {
    pub fn help_viewport_startup(&mut self, ctx: &Context) {
        if self
            .render
            .viewports
            .show_help_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let show_viewport_pointer = self.render.viewports.show_help_viewport.clone();
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
                                ui.heading("Help");
                                ui.separator();
                                ui.group(|ui: &mut Ui| {
                                    ui.heading("Overview");
                                    ui.vertical(|ui: &mut Ui| {
                                        ui.label("Urd is a journaling tool. To get started, move this window next to the main window and lets take a look at that window.");
                                        ui.label("The main window is split into three parts:");
                                        ui.indent(Id::new("main_window"), |ui: &mut Ui| {
                                            ui.label("- Top: The top panel contains buttons:");
                                            ui.indent(Id::new("top_panel"), |ui: &mut Ui| {
                                                ui.label("- Urd: Contains several buttons, all related to interacting with the program itself.");
                                                ui.indent(Id::new("urd_buttons"), |ui: &mut Ui| {
                                                    ui.label("- Settings: Opens the settings menu");
                                                    ui.indent(Id::new("settings_buttons"), |ui: &mut Ui| {
                                                        ui.label("Inside the settings menu you can customise Urd to your needs.");
                                                        ui.label("I highly recommend you to look through this menu at least once before using Urd.");
                                                    });
                                                    ui.label("- About: Opens the about window");
                                                    ui.label("- Licenses: Opens the licenses window");
                                                    ui.label("- Help: Opens this help window");
                                                });
                                                ui.label("- Journal: Contains several buttons, all related to interacting with the journal.");
                                                ui.indent(Id::new("journal_buttons"), |ui: &mut Ui| {
                                                    ui.label("- Search: Opens the search page");
                                                    ui.label("- Export: Allows you to export the journal into a directory of your choice. All entries will be exported as .txt files");
                                                    ui.indent(Id::new("export_buttons"), |ui: &mut Ui| {
                                                        ui.label("Clicking it the first time will open a setup wizard. You can open this wizard any time afterwards inside the settings menu.");
                                                    });
                                                    ui.label("- Backup: Contains two buttons:");
                                                    ui.indent(Id::new("backup_buttons"), |ui: &mut Ui| {
                                                        ui.label("- Create: Backup the journal");
                                                        ui.indent(Id::new("export_buttons"), |ui: &mut Ui| {
                                                            ui.label("Clicking it the first time will open a setup wizard. You can open this wizard any time afterwards inside the settings menu.");
                                                        });
                                                        ui.label("- Restore: Restore the journal from a backup");
                                                    });
                                                });
                                                ui.label("- Navigation: Backup navigation buttons");
                                                ui.indent(Id::new("navigation_buttons"), |ui: &mut Ui| {
                                                    ui.label("- Go to top level: Navigate to the top level inside the entry browser");
                                                    ui.label("- Go back one level: Navigate back one level inside the entry browser");
                                                })
                                            });
                                            ui.label("- Left side: The side panel contains the entry browser. It is split into two parts stacked on top of each other:");
                                            ui.indent(Id::new("left_side"), |ui: &mut Ui| {
                                                ui.label("- Top: The top part represents the current folder.");
                                                ui.indent(Id::new("top_left_side"), |ui: &mut Ui| {
                                                    ui.label("There are only 3 levels of folders inside the entry browser.");
                                                    ui.label("The top level folder contains all years, inside one of them are all months of that year, inside one of them all days of that month.");
                                                    ui.label("Clicking anywhere inside the top part will take you back one level.");
                                                });
                                                ui.label("- Bottom: The bottom part represents the entries inside the current folder.");
                                                ui.indent(Id::new("bottom_left_side"), |ui: &mut Ui| {
                                                    ui.label("This will display all entries of the currently selected folder.");
                                                    ui.label("Clicking on an entry will either open it in the side panel if it is a folder, or the main panel if it is an entry.");
                                                });
                                            });
                                            ui.label("- Right side: The right side panel contains the entry editor. It is split into four parts stacked on top of each other:");
                                            ui.indent(Id::new("right_side"), |ui: &mut Ui| {
                                                ui.label("- Top: The top part contains all possible interactions with the entry editor.");
                                                ui.indent(Id::new("top_right_side"), |ui: &mut Ui| {
                                                    ui.label("- Text colour: Change the text colour");
                                                    ui.label("- Font size: Change the font size");
                                                    ui.label("- Monospace: Toggle monospace");
                                                    ui.label("- Save entry: Save the entry, writes it to the journal and to disk");
                                                    ui.label("- Reset entry: Resets the entry, meaning that all contents will be deleted, the entry itself will not be deleted");
                                                    ui.label("- Todays entry: Creates a new entry or opens the existing entry for today");
                                                });
                                                ui.label("- Title: The title of the entry. This cannot be changed and is always the date of the entry.");
                                                ui.label("- Middle: The middle part contains the entry itself inside the entry editor.");
                                                ui.label("- Bottom: The bottom part contains the entry metadata.");
                                                ui.indent(Id::new("bottom_metadata"), |ui: &mut Ui| {
                                                    ui.label("There are 4 types of metadata. All are added into the entry itself. You can click on any one of them to open the search page and search for that tag.");
                                                    ui.indent(Id::new("metadata"), |ui: &mut Ui| {
                                                        ui.label("- Project tags: The project tags of the entry. Added by prepending the word with '+'. e.g. +Holiday");
                                                        ui.label("- Context tags: The context tags of the entry. Added by prepending the word with '@'. e.g. @Work");
                                                        ui.label("- Special tags: These tags consists of a key:value pair. You can use this to store any information you want. e.g. date_due:2025-03-15");
                                                        ui.label("- Bespoke tags: Tags for any miscellaneous information. They are added by prepending the word with '#'. e.g. #Tag");
                                                    });
                                                });
                                            });
                                        });
                                    });
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.heading("Password");
                                    ui.vertical(|ui: &mut Ui| {
                                        ui.label("Urd can be locked with a password. This can be done after setting a password inside the settings menu.");
                                        ui.label("Clicking the lock button will lock the entry editor.");
                                        ui.label("Whether Urd was locked using the button, or just started up, no journal entry will be visible until the correct password is entered.");
                                        ui.indent(Id::new("usage"), |ui: &mut Ui| {
                                            ui.label("It is important to note that the password is stored unencrypted on disk. Also the journal is stored unencrypted on disk.");
                                            ui.label("The password protection can be circumvented by simply removing the settings file.");
                                            ui.label("This protection is designed to keep honest people honest and protect from accidental data exposure.");
                                        });
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
