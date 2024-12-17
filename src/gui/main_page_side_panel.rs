use eframe::{egui::{Context, FontId, ScrollArea, SidePanel, TextBuffer, TextEdit, Ui, Vec2}, Frame};

use crate::{journal_entries::Folder, render::ShowFolder};

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
                ui.style_mut().spacing.button_padding = Vec2::from([self.settings.size.side_panel_width / 14.0, 1.0]);
                ui.group(|ui: &mut Ui| {
                    ui.horizontal(|ui: &mut Ui| {
                        if ui.button("Go back one level").clicked() {
                            self.go_back_one_level();
                        };
                        if ui.button("Go to top level").clicked() {
                            self.render.show_folder = ShowFolder::All
                        };
                    });
                }); 
            });
            ui.separator();
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                // Note for future: I wanted to split this into more functions but I couldn't get it to work
                // TLDR: Nested mutating ownership of self
                match self.render.show_folder {
                    ShowFolder::All => {
                        ui.vertical_centered_justified(|ui: &mut Ui| {
                            ui.heading("Years");
                        });
                        ui.separator();
                        let current_year = horae::Utc::now().date().year;
                        for n in &self.journal.entries {
                            debug_assert!(n.is_folder());
                            let folder = n.get_folder().unwrap();
                            let name = folder.name.as_str();
                            let display_txt = {
                                if folder.entries.len() == 12 {
                                    self.perfectly_completed(name)
                                } else if name == current_year.to_string() {
                                    self.currently_selected(name)
                                } else {
                                    self.normally_completed(name)
                                }
                            };
                            ui.vertical_centered_justified(|ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    if ui.label(display_txt).clicked() {
                                        self.render.show_folder = ShowFolder::Year(name.parse().expect("Failed to parse year, year is not a number (u16)"));
                                    }
                                });
                            });
                        }
                    }
                    ShowFolder::Year(year) => {
                        ui.vertical_centered_justified(|ui: &mut Ui| {
                            if ui.heading(year.to_string()).clicked() {
                                self.go_back_one_level();
                            }
                        });
                        ui.separator();
                        let year_folder = self.journal.entries.iter().find(|entry| entry.get_folder().unwrap().name == year.to_string()).expect("Failed to find displayed folder");
                        let months_folder = year_folder.get_folder().unwrap();
                        let current_month = horae::Utc::now().date().month;
                        for n in &months_folder.entries {
                            debug_assert!(n.is_folder());
                            let folder = n.get_folder().unwrap();
                            let name = folder.name.as_str();
                            let display_txt = {
                                if month_num_to_days(name.parse().expect("Failed to parse month, month is not a number (u8)")) == folder.entries.len() as u8 {
                                    self.perfectly_completed(name)
                                } else if name == current_month.to_string() {
                                    self.currently_selected(name)
                                } else {
                                    self.normally_completed(name)
                                }
                            };
                            ui.vertical_centered_justified(|ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    if ui.label(display_txt).clicked() {
                                        self.render.show_folder = ShowFolder::Month(year, name.parse().expect("Failed to parse month, month is not a number (u8)"));
                                    }
                                });
                            });
                        }
                    }
                    ShowFolder::Month(year, month) => {
                        ui.separator();
                        let year_folder: &mut Folder = self.journal.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == year.to_string()).expect("Failed to find displayed folder").get_folder_mut().unwrap();
                        let month_folder: &mut Folder = year_folder.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == month.to_string()).expect("Failed to find displayed folder").get_folder_mut().unwrap();
                        for n in &mut month_folder.entries {
                            debug_assert!(n.is_journal_entry());
                            let entry = n.get_journal_entry().unwrap();
                            let mut body = {
                                let tmp = entry.text.clone();
                                let mut word_store = tmp.split_whitespace().take(50).collect::<Vec<&str>>();
                                word_store.push("...");
                                word_store.join(" ")
                            };
                            ui.vertical_centered_justified(|ui: &mut Ui| {
                                if ui.group(|ui: &mut Ui| {
                                    ui.label(entry.title.as_str());
                                    ui.add_enabled(false, TextEdit::multiline(&mut body).desired_width(f32::INFINITY).text_color(self.settings.font.text_colour).font(font.clone()));
                                }).response.clicked() {
                                    self.journal.current_entry = n.get_journal_entry().unwrap().clone();
                                };
                            });
                        }
                    }
                }
            });
        });
    }

    fn go_back_one_level(&mut self) {
        match &self.render.show_folder {
            ShowFolder::All => {},
            ShowFolder::Year(_) => {
                self.render.show_folder = ShowFolder::All
            },
            ShowFolder::Month(year, _) => {
                self.render.show_folder = ShowFolder::Year(*year)
            },
        }
    }
    
    fn currently_selected(&self, name: &str) -> String {
        format!("{} {} {}", self.settings.gui.file_marker_currently.start, name, self.settings.gui.file_marker_currently.end)
    }

    fn perfectly_completed(&self, name: &str) -> String {
        format!("{} {} {}", self.settings.gui.file_marker_perfectly.start, name, self.settings.gui.file_marker_perfectly.end)
    }

    fn normally_completed(&self, name: &str) -> String {
        format!("{} {} {}", self.settings.gui.file_marker_normally.start, name, self.settings.gui.file_marker_normally.end)
    }
}

fn month_num_to_name(month: u8) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => unreachable!(),
    }
}

fn month_num_to_days(month: u8) -> u8 {
    debug_assert!(month > 0 && month < 13);
    match month {
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}