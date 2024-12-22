use eframe::{
    egui::{CentralPanel, Ui},
    *,
};
use egui::{Align, FontId, ScrollArea, TextEdit};
use nabu::Object;

use crate::{error::Error, journal_entries::{EntryType, Folder}, settings::{MAX_FONT_SIZE, MIN_FONT_SIZE}};

use super::UrdState;

impl UrdState {
    pub fn main_page(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if self
            .render
            .viewports
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
                ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                    ui.add(TextEdit::multiline(&mut self.journal.current_entry.text)
                        .horizontal_align(Align::Center)
                        .lock_focus(true)
                        .text_color(self.settings.font.text_colour)
                        .font(font.clone())
                    );
                    // TODO: tmp code below, add clicking on a tag to search for it in the journal
                    // when search is done
                    ui.group(|ui: &mut Ui| {
                        let tmp_project_tags = {
                            let bind = self.journal.current_entry.metadata.get("project_tags").unwrap().into_array();
                            if bind.is_none() {
                                vec![]
                            } else {
                                bind.unwrap().into_vec()
                            }
                        };
                        let project_tags_as_txt = tmp_project_tags.iter().map(|tag| tag.into_string().unwrap()).collect::<Vec<String>>().join(", ");
                        ui.horizontal(|ui: &mut Ui| {
                            ui.group(|ui: &mut Ui| {
                                ui.label("Project Tags: ");
                                ui.label(project_tags_as_txt);
                            })

                        });
                        let tmp_context_tags = {
                            
                            let bind = self.journal.current_entry.metadata.get("context_tags").unwrap().into_array();
                            if bind.is_none() {
                                vec![]
                            } else {
                                bind.unwrap().into_vec()
                            }
                        };
                        let context_tags_as_txt = tmp_context_tags.iter().map(|tag| tag.into_string().unwrap()).collect::<Vec<String>>().join(", ");
                        ui.label(format!("Context Tags: {}", context_tags_as_txt));
                        let tmp_special_tags = {
                            let bind = self.journal.current_entry.metadata.get("special_tags").unwrap().into_object();
                            if bind.is_none() {
                                Object::new()
                            } else {
                                bind.unwrap()
                            }
                        };
                        let special_tags_as_txt = tmp_special_tags.iter().map(|(key, value)| format!("{}:{}", key, value.into_string().unwrap())).collect::<Vec<String>>().join(", ");
                        ui.label(format!("Special Tags: {}", special_tags_as_txt));
                    }).response
                });
            })
        });
    }

    fn central_panel_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.group(|ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Text Colour: ");
                    if ui.color_edit_button_srgba(&mut self.settings.font.text_colour).changed() {
                        let save = self.settings.save();
                        if save.is_err() {
                            self.error = Error::new(save.unwrap_err().to_string());
                        }
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
            if ui.button("Save entry").clicked() {
                self.save_entry_to_journal();
                let save = self.journal.save();
                if save.is_err() {
                    self.error = Error::new(save.unwrap_err().to_string());
                }
            };
            if ui.button("Delete entry").clicked() {
                self.delete_entry_from_journal();
                let save = self.journal.save();
                if save.is_err() {
                    self.error = Error::new(save.unwrap_err().to_string());
                }
            };
        });
    }

    fn save_entry_to_journal(&mut self) {
        let save = self.settings.save();
        if save.is_err() {
            self.error = Error::new(save.unwrap_err().to_string());
        }
        self.journal.current_entry.overwrite(self.journal.current_entry.text.clone());
        // TODO: save journal entry
        // First search for its position in journal
        // Then save / overwrite file with same name
        let (year, month) = {
            let date = self.journal.current_entry.metadata.get("date").unwrap().into_object().unwrap();
            let year = date.get("year").unwrap().into_number().unwrap().into_usize().unwrap();
            let month = date.get("month").unwrap().into_number().unwrap().into_usize().unwrap();
            (year, month)
        };
        let tmp_year_folder = self.journal.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == year.to_string());
        if tmp_year_folder.is_none() {
            let mut year_folder = Folder::new(year.to_string());
            let mut month_folder = Folder::new(month.to_string());
            month_folder.entries.push_front(EntryType::JournalEntry(self.journal.current_entry.clone()));
            year_folder.entries.push_front(EntryType::Folder(month_folder));
            self.journal.entries.push_front(EntryType::Folder(year_folder));
        } else {
            let year_folder = tmp_year_folder.unwrap().get_folder_mut().unwrap();
            let tmp_month_folder = year_folder.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == month.to_string());
            if tmp_month_folder.is_none() {
                let mut month_folder = Folder::new(month.to_string());
                month_folder.entries.push_front(EntryType::JournalEntry(self.journal.current_entry.clone()));
                year_folder.entries.push_front(EntryType::Folder(month_folder));
            } else {
                let month_folder = tmp_month_folder.unwrap().get_folder_mut().unwrap();
                let day_search = month_folder.entries.iter_mut().find(|entry| entry.get_journal_entry().unwrap().title == self.journal.current_entry.title);
                if day_search.is_none() {
                    month_folder.entries.push_front(EntryType::JournalEntry(self.journal.current_entry.clone()));
                } else {
                    // TODO: Fix this
                    println!("Day already exists");
                    let day_folder = day_search.unwrap().get_journal_entry_mut();
                    if day_folder.is_some() {
                        day_folder.unwrap().overwrite(self.journal.current_entry.text.clone());
                    }
                }
            }
        }
    }

    fn delete_entry_from_journal(&mut self) {
        let (year, month) = {
            let date = self.journal.current_entry.metadata.get("date").unwrap().into_object().unwrap();
            let year = date.get("year").unwrap().into_number().unwrap().into_usize().unwrap();
            let month = date.get("month").unwrap().into_number().unwrap().into_usize().unwrap();
            (year, month)
        };
        let tmp_year_folder = self.journal.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == year.to_string());
        if tmp_year_folder.is_some() {
            let year_folder = tmp_year_folder.unwrap().get_folder_mut().unwrap();
            let tmp_month_folder = year_folder.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == month.to_string());
            if tmp_month_folder.is_some() {
                let month_folder = tmp_month_folder.unwrap().get_folder_mut().unwrap();
                let actual_entry = month_folder.entries.iter_mut().find(|entry| entry.get_journal_entry().unwrap().title == self.journal.current_entry.title);
                if actual_entry.is_some() {
                    actual_entry.unwrap().get_journal_entry_mut().unwrap().reset();
                }
            }
        }
        self.journal.current_entry.text = "".to_string();
    }
}
