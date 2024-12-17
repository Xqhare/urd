use eframe::{
    egui::{CentralPanel, Ui},
    *,
};
use egui::{Align, FontId, ScrollArea, TextEdit};

use crate::settings::{MAX_FONT_SIZE, MIN_FONT_SIZE};

use super::UrdState;

impl UrdState {
    pub fn main_page(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if self
            .settings
            .show_settings_viewport
        {
            self.settings_viewport_startup(ctx);
        } else {
            self.main_side_panel(ctx, frame);
        }
        // Remember, central panel last
        self.main_central_panel(ctx, frame);
    }

    fn main_central_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let font = {
            if self.settings.font.monospace {
                FontId::monospace(self.settings.font.size)
            } else {
                FontId::proportional(self.settings.font.size)
            }
        };
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            self.central_panel_menu(ui);
            ui.separator();
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered_justified(|ui: &mut Ui| {
                    ui.heading(&self.journal.current_entry.title);
                });
                ui.separator();
                ui.add_sized(
                    ui.available_size(),
                    TextEdit::multiline(&mut self.journal.current_entry.text)
                        .horizontal_align(Align::Center)
                        .lock_focus(true)
                        .text_color(self.settings.font.text_colour)
                        .font(font.clone()),
                );
                /* if title.response.lost_focus() || text_edit.lost_focus() {
                    // TODO: save journal entry
                    // this saves only if the focus leaves the text box
                    println!("testing lost focus");
                    self.save_journal_entry();
                } */
            })
        });
    }

    fn central_panel_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.group(|ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Text Colour: ");
                    if ui.color_edit_button_srgba(&mut self.settings.font.text_colour).changed() {
                        self.settings.save();
                    };
                })
            });
            ui.group(|ui: &mut Ui| {
                ui.label("Font Size: ");
                ui.add(egui::Slider::new(&mut self.settings.font.size, MIN_FONT_SIZE..=MAX_FONT_SIZE));
            });
            ui.group(|ui: &mut Ui| {
                ui.checkbox(&mut self.settings.font.monospace, "Monospace");
            });
            if ui.button("Save").clicked() {
                //self.save_journal_entry();
                let t = self.settings.save();
                println!("{:?}", t);
            };
            if ui.button("Delete entry").clicked() {
                self.delete_journal_entry();
            };
        });
    }
}
