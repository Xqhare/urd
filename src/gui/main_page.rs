use eframe::{
    egui::{CentralPanel, Ui},
    *,
};
use egui::{Align, FontId, ScrollArea, SidePanel, TextEdit, TopBottomPanel};

use super::UrdState;

impl UrdState {
    pub fn main_page(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.main_top_panel(ctx, frame);
        self.main_side_panel(ctx, frame);
        // Remember, central panel last
        self.main_central_panel(ctx, frame);
    }

    fn main_top_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                if ui.button("Settings").clicked() {
                    self.settings
                        .show_settings_viewport
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                }
                if ui.button("About").clicked() {
                    self.show_about_viewport
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                }
                if ui.button("Licenses").clicked() {
                    self.show_licenses_viewport
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                }
                if ui.button("Help").clicked() {
                    self.show_help_viewport
                        .store(true, std::sync::atomic::Ordering::Relaxed);
                }
            });
        });
        if self
            .settings
            .show_settings_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.settings_viewport_startup(ctx);
        }
        if self
            .show_about_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.about_viewport_startup(ctx);
        }
        if self
            .show_licenses_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.licenses_viewport_startup(ctx);
        }
        if self
            .show_help_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            self.help_viewport_startup(ctx);
        }
    }

    fn main_side_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let font = {
            if self.settings.font.monospace {
                FontId::monospace(self.settings.font.size)
            } else {
                FontId::proportional(self.settings.font.size)
            }
        };
        SidePanel::left("entry_browser").default_width(self.settings.size.side_panel_width).show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                for (index, entry) in self.journal.entries.iter_mut().enumerate() {
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
                }
                ui.separator();
            });
        });
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
                let _title = TextEdit::singleline(&mut self.journal.current_entry.title)
                    .horizontal_align(Align::Center)
                    .frame(false)
                    .desired_width(f32::INFINITY)
                    .text_color(self.settings.font.text_colour)
                    .font(font.clone())
                    .show(ui);
                let _text_edit = ui.add_sized(
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
                    ui.color_edit_button_srgba(&mut self.settings.font.text_colour);
                })
            });
            ui.group(|ui: &mut Ui| {
                ui.label("Font Size: ");
                ui.add(egui::Slider::new(&mut self.settings.font.size, 8.0..=48.0));
            });
            ui.group(|ui: &mut Ui| {
                ui.checkbox(&mut self.settings.font.monospace, "Monospace");
            });
            if ui.button("Save").clicked() {
                self.save_journal_entry();
            };
            if ui.button("Delete entry").clicked() {
                self.delete_journal_entry();
            };
        });
    }
}
