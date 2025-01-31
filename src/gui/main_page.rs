use eframe::{
    egui::{CentralPanel, Ui},
    *,
};
use egui::{Align, ComboBox, FontId, Margin, ScrollArea, TextEdit};
use nabu::{Object, XffValue};

use crate::{
    error::Error,
    journal_entries::{EntryType, Folder, JournalEntry},
    moods::Mood,
    settings::{MAX_FONT_SIZE, MIN_FONT_SIZE},
};

use super::UrdState;

impl UrdState {
    pub fn main_page(&mut self, ctx: &egui::Context) {
        if self.render.view.pages.show_settings_page {
            self.settings_viewport_startup(ctx);
        } else {
            self.main_side_panel(ctx);
        }
        // Remember, central panel last
        self.main_central_panel(ctx);
    }

    fn main_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            self.main_inside_panel(ui);
        });
    }

    pub fn main_inside_panel(&mut self, ui: &mut Ui) {
        let font = {
            if self.settings.font.monospace {
                FontId::monospace(self.settings.font.size)
            } else {
                FontId::proportional(self.settings.font.size)
            }
        };
        self.central_panel_menu(ui);
        ui.separator();
        ScrollArea::vertical().show(ui, |ui: &mut Ui| {
            ui.vertical_centered_justified(|ui: &mut Ui| {
                //DEBUG LEAVE FOR LATER
                //ui.add(TextEdit::singleline(&mut self.journal.current_entry.title).horizontal_align(Align::Center));
                ui.heading(&self.journal.current_entry.title).on_hover_text("The title of this entry. It is the date of the entry and cannot be changed");
            });
            ui.separator();
            ui.horizontal(|ui: &mut Ui| {
                if !self.render.view.ui_state.show_add_mood_field {
                    ui.add_space(ui.available_width() / 3.95);
                }
                self.mood(ui);
                ui.separator();
                // Again hacky, but saves me from declaring another state field
                let mut tmp_bool = self
                    .journal
                    .current_entry
                    .metadata
                    .get_mut("important_day")
                    .unwrap()
                    .into_boolean()
                    .unwrap();
                if ui.checkbox(&mut tmp_bool, "Important Day").on_hover_text("Mark this day as important to find it easier").changed() {
                    self.journal
                        .current_entry
                        .metadata
                        .insert("important_day".to_string(), XffValue::from(tmp_bool));
                    println!(
                        "{}",
                        self.journal
                            .current_entry
                            .metadata
                            .get("important_day")
                            .unwrap()
                            .into_boolean()
                            .unwrap()
                    );
                }
            });
            ui.separator();
            ui.add_sized(ui.available_size(), |ui: &mut Ui| {
                let entry_text = ui.add(
                    TextEdit::multiline(&mut self.journal.current_entry.text)
                        .horizontal_align(Align::LEFT)
                        .lock_focus(true)
                        .text_color(self.settings.font.text_colour)
                        .font(font.clone())
                        .margin(Margin::same(5.0))
                        .hint_text("Write your journal entry here")
                        .desired_width(f32::INFINITY)
                );
                if entry_text.changed() {
                    if let Err(err) = self.journal.save() {
                        self.error = Error::new(
                            err.to_string(),
                            "Writing journal to disk failed.".to_string(),
                        );
                    }
                }
                entry_text
            });
            ui.group(|ui: &mut Ui| {
                ui.vertical_centered_justified(|ui: &mut Ui| {
                    ui.heading("Metadata").on_hover_text("The metadata of this entry; Project tags, Context tags, Special tags and Bespoke tags");
                    ui.horizontal(|ui: &mut Ui| {
                        let tmp_project_tags = {
                            let bind = self
                                .journal
                                .current_entry
                                .metadata
                                .get("project_tags")
                                .unwrap()
                                .into_array();
                            if let Some(tags) = bind {
                                tags.into_vec()
                            } else {
                                vec![]
                            }
                        };
                        let project_tags_as_txt = tmp_project_tags
                            .iter()
                            .map(|tag| tag.into_string().unwrap())
                            .collect::<Vec<String>>();
                        ui.add_sized(
                            [ui.available_width(), ui.available_height()],
                            |ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    ui.vertical(|ui: &mut Ui| {
                                        ui.heading("Project Tags").on_hover_text("The project tags of this entry. They are added by prepending the word with `+`");
                                        if project_tags_as_txt.is_empty() {
                                            ui.label("Add with `+{tag}`");
                                        } else {
                                            for tag in project_tags_as_txt {
                                                ui.group(|ui: &mut Ui| {
                                                    if ui.label(&tag).on_hover_text("Click to search").clicked() {
                                                        self.search.query =
                                                            format!("{}, ", tag);
                                                        self.search_current_query();
                                                        self.render
                                                            .view
                                                            .pages
                                                            .show_search_page = true;
                                                    }
                                                });
                                            }
                                        }
                                    })
                                })
                                .response
                            },
                        );

                        let tmp_context_tags = {
                            let bind = self
                                .journal
                                .current_entry
                                .metadata
                                .get("context_tags")
                                .unwrap()
                                .into_array();
                            if let Some(tags) = bind {
                                tags.into_vec()
                            } else {
                                vec![]
                            }
                        };
                        let context_tags_as_txt = tmp_context_tags
                            .iter()
                            .map(|tag| tag.into_string().unwrap())
                            .collect::<Vec<String>>();
                        ui.add_sized(
                            [ui.available_width() * 1.95, ui.available_height()],
                            |ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    ui.vertical(|ui: &mut Ui| {
                                        ui.heading("Context Tags").on_hover_text("The context tags of this entry. They are added by prepending the word with `@`");
                                        if context_tags_as_txt.is_empty() {
                                            ui.label("Add with `@{tag}`");
                                        } else {
                                            for tag in context_tags_as_txt {
                                                ui.group(|ui: &mut Ui| {
                                                    if ui.label(&tag).on_hover_text("Click to search").clicked() {
                                                        self.search.query =
                                                            format!("{}, ", tag);
                                                        self.search_current_query();
                                                        self.render
                                                            .view
                                                            .pages
                                                            .show_search_page = true;
                                                    }
                                                });
                                            }
                                        }
                                    });
                                })
                                .response
                            },
                        );
                    });

                    ui.horizontal(|ui: &mut Ui| {
                        let tmp_special_tags = {
                            let bind = self
                                .journal
                                .current_entry
                                .metadata
                                .get("special_tags")
                                .unwrap()
                                .into_object();
                            if let Some(bind) = bind {
                                bind
                            } else {
                                Object::new()
                            }
                        };
                        let special_tags_as_txt = tmp_special_tags
                            .iter()
                            .map(|(key, value)| {
                                format!("{}:{}", key, value.into_string().unwrap())
                            })
                            .collect::<Vec<String>>();
                        ui.add_sized(
                            [ui.available_width(), ui.available_height()],
                            |ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    ui.vertical(|ui: &mut Ui| {
                                        ui.heading("Special Tags").on_hover_text("The special tags of this entry. They are added by prepending the word with `{key}:{value}`");
                                        if special_tags_as_txt.is_empty() {
                                            ui.label("Add with `{key}:{value}`");
                                        } else {
                                            for tag in special_tags_as_txt {
                                                ui.group(|ui: &mut Ui| {
                                                    if ui.label(&tag).on_hover_text("Click to search key").clicked() {
                                                        self.search.query =
                                                            format!("{}, ", tag.split(":").next().unwrap());
                                                        self.search_current_query();
                                                        self.render
                                                            .view
                                                            .pages
                                                            .show_search_page = true;
                                                    }
                                                });
                                            }
                                        }
                                    });
                                })
                                .response
                            },
                        );
                        let tmp_bespoke_tags = {
                            let bind = self
                                .journal
                                .current_entry
                                .metadata
                                .get("bespoke_tags")
                                .unwrap()
                                .into_array();
                            if let Some(tags) = bind {
                                tags.into_vec()
                            } else {
                                vec![]
                            }
                        };
                        let bespoke_tags_as_txt = tmp_bespoke_tags
                            .iter()
                            .map(|tag| tag.into_string().unwrap())
                            .collect::<Vec<String>>();
                        ui.add_sized(
                            [ui.available_width() * 1.95, ui.available_height()],
                            |ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    ui.vertical(|ui: &mut Ui| {
                                        ui.heading("Bespoke Tags").on_hover_text("The bespoke tags of this entry. They are added by prepending the word with `#{value}`");
                                        if bespoke_tags_as_txt.is_empty() {
                                            ui.label("Add with `#{value}`");
                                        } else {
                                            for tag in bespoke_tags_as_txt {
                                                ui.group(|ui: &mut Ui| {
                                                    if ui.label(&tag).on_hover_text("Click to search").clicked() {
                                                        self.search.query =
                                                            format!("{}, ", tag);
                                                        self.search_current_query();
                                                        self.render
                                                            .view
                                                            .pages
                                                            .show_search_page = true;
                                                    }
                                                });
                                            }
                                        }
                                    });
                                })
                                .response
                            },
                        );
                    });
                });
            });
        });
    }

    fn mood(&mut self, ui: &mut Ui) {
        ComboBox::from_label("Mood")
            .selected_text(
                self.journal
                    .current_entry
                    .metadata
                    .get("mood")
                    .unwrap()
                    .into_string()
                    .unwrap(),
            )
            .show_ui(ui, |ui: &mut Ui| {
                for (mood, _) in self.journal.moods.iter() {
                    // Hacky af, I know - but hey I can save here too!
                    if ui
                        .selectable_value(
                            &mut self
                                .journal
                                .current_entry
                                .metadata
                                .get("mood")
                                .unwrap()
                                .into_string()
                                .unwrap(),
                            mood.to_string(),
                            mood,
                        )
                        .changed()
                    {
                        self.journal
                            .current_entry
                            .metadata
                            .insert("mood".to_string(), XffValue::from(mood));
                        if let Err(err) = self.journal.save() {
                            self.error = Error::new(
                                err.to_string(),
                                "Writing journal to disk failed.".to_string(),
                            );
                        }
                    };
                }
            })
            .response
            .on_hover_text("Select the mood of this entry");
        if self.render.view.ui_state.show_add_mood_field {
            ui.text_edit_singleline(&mut self.state_store.new_mood.name)
                .on_hover_text("Enter the name of the new mood");
            ui.label("Mood Colour: ");
            ui.color_edit_button_srgba(&mut self.state_store.new_mood.colour)
                .on_hover_text("Choose the colour of the new mood");

            if ui
                .button("Add mood")
                .on_hover_text("Save the custom mood")
                .clicked()
            {
                if self
                    .journal
                    .moods
                    .contains_key(&self.state_store.new_mood.name)
                {
                    self.error = Error::new(
                        "Mood already exists.".to_string(),
                        "Please choose a different name.".to_string(),
                    );
                } else {
                    self.journal.moods.insert(
                        self.state_store.new_mood.name.clone(),
                        self.state_store.new_mood.colour.to_array().to_vec(),
                    );
                    if let Err(err) = self.journal.save() {
                        self.error = Error::new(
                            err.to_string(),
                            "Writing journal to disk failed.".to_string(),
                        );
                    }
                    // Reset
                    self.state_store.new_mood = Mood::default();
                    self.render.view.ui_state.show_add_mood_field = false;
                }
            };
        } else if ui
            .button("Add mood")
            .on_hover_text("Add a custom mood")
            .clicked()
        {
            self.render.view.ui_state.show_add_mood_field = true;
            self.state_store.new_mood.name = "Custom Mood".to_string();
        };
    }

    fn central_panel_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui: &mut Ui| {
            ui.group(|ui: &mut Ui| {
                ui.horizontal(|ui: &mut Ui| {
                    ui.label("Text Colour: ")
                        .on_hover_text("Click to change the text colour");
                    if ui
                        .color_edit_button_srgba(&mut self.settings.font.text_colour)
                        .on_hover_text("Click to change the text colour")
                        .changed()
                    {
                        if let Err(err) = self.settings.save() {
                            self.error = Error::new(
                                err.to_string(),
                                "Writing settings to disk failed.".to_string(),
                            );
                        }
                    };
                })
            });
            ui.group(|ui: &mut Ui| {
                ui.label("Font Size: ")
                    .on_hover_text("Drag and drop or enter a number to change the font size");
                ui.add(egui::Slider::new(
                    &mut self.settings.font.size,
                    MIN_FONT_SIZE..=MAX_FONT_SIZE,
                ))
                .on_hover_text("Drag and drop or enter a number to change the font size");
            });
            ui.group(|ui: &mut Ui| {
                ui.checkbox(&mut self.settings.font.monospace, "Monospace")
                    .on_hover_text("Toggle monospace font");
            });
            if ui
                .button("Save entry")
                .on_hover_text("Save the current entry")
                .clicked()
            {
                self.save_entry_to_journal();
                if let Err(err) = self.journal.save() {
                    self.error = Error::new(
                        err.to_string(),
                        "Writing journal to disk failed.".to_string(),
                    );
                }
            };
            //DEBUG LEAVE FOR LATER
            /* if ui.button("Debug Create").clicked() {
                let date_split = self.journal.current_entry.title.split("-").collect::<Vec<&str>>();
                debug_assert!(date_split.len() == 3);
                let year = date_split[0].parse::<u16>().unwrap();
                let month = date_split[1].parse::<u8>().unwrap();
                let day = date_split[2].parse::<u8>().unwrap();
                let date = {
                    let mut out = Object::new();
                    out.insert("year", year);
                    out.insert("month", month);
                    out.insert("day", day);
                    out
                };
                self.journal.current_entry.metadata.insert("date".to_string(), XffValue::from(date));

                self.save_entry_to_journal();
                let save = self.journal.save();
                if save.is_err() {
                    self.error = Error::new(
                        save.unwrap_err().to_string(),
                        "Writing journal to disk failed.".to_string(),
                    );
                }
                let next_day = day + 1;
                let next_date = format!("{}-{:02}-{:02}", year, month, next_day);
                self.journal.current_entry.title = next_date;
            }; */
            if ui
                .button("Reset entry")
                .on_hover_text(
                    "Reset the current entry. Deletes all text and leaves the entry empty",
                )
                .clicked()
            {
                self.delete_entry_from_journal();
                if let Err(err) = self.journal.save() {
                    self.error = Error::new(
                        err.to_string(),
                        "Writing journal to disk failed.".to_string(),
                    );
                }
            };
            // Fallback option, if urd is kept open for a long time (the date has changed since
            // startup) no new entry will be generated automatically - this will create such a new entry, but
            // only if it does not already exist. This also loads that entry.
            if ui
                .button("Todays entry")
                .on_hover_text("Creates a new entry or opens the existing entry for today")
                .clicked()
            {
                let date_today = {
                    let mut tmp = horae::Utc::now();
                    tmp.with_timezone(self.settings.timezone.timezone);
                    tmp
                };
                let tmp_year_folder = self
                    .journal
                    .entries
                    .front_mut()
                    .unwrap()
                    .get_folder_mut()
                    .unwrap();
                if tmp_year_folder.name == date_today.date().year.to_string() {
                    let tmp_month_folder = tmp_year_folder
                        .entries
                        .front_mut()
                        .unwrap()
                        .get_folder_mut()
                        .unwrap();
                    if tmp_month_folder.name == date_today.date().month.to_string() {
                        for entry in tmp_month_folder.entries.clone() {
                            if entry.get_journal_entry().unwrap().title
                                == date_today.date().to_string()
                            {
                                self.journal.current_entry =
                                    entry.get_journal_entry().unwrap().clone();
                                break;
                            } else {
                                // Day not found, but month and year were found
                                let new_journal_entry = JournalEntry::new(&self.settings);
                                tmp_month_folder
                                    .entries
                                    .push_front(EntryType::JournalEntry(new_journal_entry.clone()));
                                self.journal.current_entry = new_journal_entry;
                            }
                        }
                    } else {
                        // Month not found, but year was found
                        let mut new_month_folder = Folder::new(date_today.date().month.to_string());
                        let new_journal_entry = JournalEntry::new(&self.settings);
                        new_month_folder
                            .entries
                            .push_front(EntryType::JournalEntry(new_journal_entry.clone()));
                        tmp_year_folder
                            .entries
                            .push_front(EntryType::Folder(new_month_folder));
                        self.journal.current_entry = new_journal_entry;
                    }
                } else {
                    // Year not found
                    let mut new_year_folder = Folder::new(date_today.date().year.to_string());
                    let mut new_month_folder = Folder::new(date_today.date().month.to_string());
                    let new_journal_entry = JournalEntry::new(&self.settings);
                    new_month_folder
                        .entries
                        .push_front(EntryType::JournalEntry(new_journal_entry.clone()));
                    new_year_folder
                        .entries
                        .push_front(EntryType::Folder(new_month_folder));
                    self.journal
                        .entries
                        .push_front(EntryType::Folder(new_year_folder));
                    self.journal.current_entry = new_journal_entry;
                }
            };
            if ui.button("Save entry and exit Urd").on_hover_text("Save entry and exit Urd").clicked() {
                let _ = self.journal.save();
                let _ = self.settings.save();
                std::process::exit(0);
            };
        });
    }

    fn save_entry_to_journal(&mut self) {
        let (year, month) = {
            let date = self
                .journal
                .current_entry
                .metadata
                .get("date")
                .unwrap()
                .into_object()
                .unwrap();
            let year = date
                .get("year")
                .unwrap()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap();
            let month = date
                .get("month")
                .unwrap()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap();
            (year, month)
        };
        let tmp_year_folder = self
            .journal
            .entries
            .iter_mut()
            .find(|entry| entry.get_folder().unwrap().name == year.to_string());
        if tmp_year_folder.is_none() {
            let mut year_folder = Folder::new(year.to_string());
            let mut month_folder = Folder::new(month.to_string());
            month_folder
                .entries
                .push_front(EntryType::JournalEntry(self.journal.current_entry.clone()));
            year_folder
                .entries
                .push_front(EntryType::Folder(month_folder));
            self.journal
                .entries
                .push_front(EntryType::Folder(year_folder));
        } else {
            let year_folder = tmp_year_folder.unwrap().get_folder_mut().unwrap();
            let tmp_month_folder = year_folder
                .entries
                .iter_mut()
                .find(|entry| entry.get_folder().unwrap().name == month.to_string());
            if tmp_month_folder.is_none() {
                let mut month_folder = Folder::new(month.to_string());
                month_folder
                    .entries
                    .push_front(EntryType::JournalEntry(self.journal.current_entry.clone()));
                year_folder
                    .entries
                    .push_front(EntryType::Folder(month_folder));
            } else {
                let month_folder = tmp_month_folder.unwrap().get_folder_mut().unwrap();
                let day_search = month_folder.entries.iter_mut().find(|entry| {
                    entry.get_journal_entry().unwrap().title == self.journal.current_entry.title
                });
                if day_search.is_none() {
                    month_folder
                        .entries
                        .push_front(EntryType::JournalEntry(self.journal.current_entry.clone()));
                } else {
                    let day_entry = day_search.unwrap().get_journal_entry_mut();
                    if let Some(entry) = day_entry {
                        entry.overwrite(
                            self.journal.current_entry.text.clone(),
                            self.journal.current_entry.metadata.clone(),
                        );
                    }
                }
            }
        }
        if let Err(err) = self.journal.save() {
            self.error = Error::new(
                err.to_string(),
                "Writing journal to disk failed.".to_string(),
            );
        }
    }

    fn delete_entry_from_journal(&mut self) {
        let (year, month) = {
            let date = self
                .journal
                .current_entry
                .metadata
                .get("date")
                .unwrap()
                .into_object()
                .unwrap();
            let year = date
                .get("year")
                .unwrap()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap();
            let month = date
                .get("month")
                .unwrap()
                .into_number()
                .unwrap()
                .into_usize()
                .unwrap();
            (year, month)
        };
        let tmp_year_folder = self
            .journal
            .entries
            .iter_mut()
            .find(|entry| entry.get_folder().unwrap().name == year.to_string());
        if tmp_year_folder.is_some() {
            let year_folder = tmp_year_folder.unwrap().get_folder_mut().unwrap();
            let tmp_month_folder = year_folder
                .entries
                .iter_mut()
                .find(|entry| entry.get_folder().unwrap().name == month.to_string());
            if tmp_month_folder.is_some() {
                let month_folder = tmp_month_folder.unwrap().get_folder_mut().unwrap();
                let actual_entry = month_folder.entries.iter_mut().find(|entry| {
                    entry.get_journal_entry().unwrap().title == self.journal.current_entry.title
                });
                if let Some(entry) = actual_entry {
                    entry.get_journal_entry_mut().unwrap().reset();
                }
            }
        }
        self.journal.current_entry.text = "".to_string();
        if let Err(err) = self.journal.save() {
            self.error = Error::new(
                err.to_string(),
                "Writing journal to disk failed.".to_string(),
            );
        }
    }
}
