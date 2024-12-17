use eframe::{egui::{Context, FontId, Grid, ScrollArea, SidePanel, Ui, Vec2}, Frame};

use crate::render::ShowFolder;

use super::UrdState;

impl UrdState {
    pub fn main_side_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        let font = {
            if self.settings.font.monospace {
                FontId::monospace(self.settings.font.size)
            } else {
                FontId::proportional(self.settings.font.size)
            }
        };
        SidePanel::left("entry_browser").min_width(self.settings.size.side_panel_width).show(ctx, |ui: &mut Ui| {
            // add space to justify with main panel settings
            ui.add_space(6.0);
            ui.scope(|ui: &mut Ui| {
                ui.style_mut().spacing.button_padding = Vec2::from([self.settings.size.side_panel_width / 14.0, 0.0]);
                ui.group(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.button("Go back one level").clicked() {
                        };
                        if ui.button("Go to top level").clicked() {
                        };
                    });
                }); 
            });
            ui.separator();
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                match &self.render.show_folder {
                    ShowFolder::All => {
                        ui.label("All");
                        for n in 0..100 {
                            ui.label(format!("Entry {}", n));
                        }
                    }
                    ShowFolder::Year(year) => {
                        ui.label(format!("Year: {}", year));
                    }
                    ShowFolder::Month(year, month) => {
                        ui.label(format!("Month: {}-{}", year, month));
                    }
                }
                // TODO: Rework with new journal
                /* for (index, entry) in self.journal.entries.iter_mut().enumerate() {
                    let entry_reaction = ui.add(|ui: &mut Ui| {
                        let group = ui.group(|ui: &mut Ui| {
                            ui.add_enabled(false, |ui: &mut Ui| {
                                TextEdit::singleline(&mut entry.title).frame(false).desired_width(f32::INFINITY).text_color(self.settings.font.text_colour).font(font.clone()).show(ui).response
                            });
                            ui.add_enabled(false, |ui: &mut Ui| {
                                TextEdit::multiline(&mut entry.text).frame(false).desired_width(f32::INFINITY).text_color(self.settings.font.text_colour).font(font.clone()).show(ui).response
                            })
                        });
                        group.response
                    });
                    // TODO: open this journal entry if clicked
                    if entry_reaction.interact(egui::Sense::click()).clicked() {
                        println!("Entry reaction! {:?} clicked", entry);
                    }
                } */
            });
        });
    }

    fn render_entry(&mut self, ctx: &Context, frame: &mut Frame, title: &str, text: &str) {
        
    }
}
