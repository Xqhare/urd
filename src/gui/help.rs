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
                                            ui.label("")
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

                                            });
                                            ui.collapsing("Entry editor", |ui: &mut Ui| {
                                                
                                            });
                                        });
                                    });
                                });
                                ui.group(|ui: &mut Ui| {
                                    ui.label("Features");
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
