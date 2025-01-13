use eframe::egui::{Context, ScrollArea, SidePanel, Ui};

use crate::{journal_entries::Folder, render::ShowFolder};

use super::UrdState;

impl UrdState {
    pub fn main_side_panel(&mut self, ctx: &Context) {
        SidePanel::left("entry_browser").min_width(self.settings.size.side_panel_width).show(ctx, |ui: &mut Ui| {
            // Note for future: I wanted to split this into more functions but I couldn't get it to work
            // TLDR: Nested mutating ownership of self
            match self.render.show_folder {
                ShowFolder::All => {
                    ui.vertical_centered_justified(|ui: &mut Ui| {
                        ui.add_space(22.5);
                            ui.heading("Years").on_hover_text("You are at the root of the journal");
                        ui.add_space(22.5);
                    }).response.on_hover_text("You are at the root of the journal");
                    ui.separator();
                    ScrollArea::vertical().show(ui, |ui: &mut Ui| {
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
                                    if ui.label(display_txt).on_hover_text("Click to open").clicked() {
                                        self.render.show_folder = ShowFolder::Year(name.parse().expect("Failed to parse year, year is not a number (u16)"));
                                    }
                                });
                            });
                        }
                    });
                }
                ShowFolder::Year(year) => {
                    ui.vertical_centered_justified(|ui: &mut Ui| {
                        let year_folder = self.journal.entries.iter().find(|entry| entry.get_folder().unwrap().name == year.to_string()).expect("Failed to find displayed folder");
                        let aspir = year_folder.get_folder().unwrap().aspirations.clone();
                        if aspir.is_null() {
                            ui.add_space(22.5);
                            if ui.heading(year.to_string()).on_hover_text("Click to go back").clicked() {
                                self.go_back_one_level();
                            }
                            ui.add_space(22.5);
                        } else {
                            let aspirations = aspir.into_object().unwrap();
                            if aspirations.len() == 0 {
                                ui.add_space(22.5);
                                if ui.heading(year.to_string()).on_hover_text("Click to go back").clicked() {
                                    self.go_back_one_level();
                                }
                                ui.add_space(22.5);
                            } else {
                                let theme = aspirations.get("theme").unwrap().into_string().unwrap();
                                let pledge = aspirations.get("pledge").unwrap().into_string().unwrap();
                                let asps = aspirations.get("resolutions").unwrap().into_array().unwrap();
                                if theme == "" && pledge == "" && asps.len() == 0 {
                                    ui.add_space(22.5);
                                    if ui.heading(year.to_string()).on_hover_text("Click to go back").clicked() {
                                        self.go_back_one_level();
                                    }
                                    ui.add_space(22.5);
                                } else {
                                    ui.add_space(2.5);
                                    if ui.heading(format!("{}", year.to_string())).on_hover_text("Click to go back").clicked() {
                                        self.go_back_one_level();
                                    }
                                    ui.add_space(2.5);
                                    ui.label(format!("The year of {}", theme)).on_hover_text("Your theme for this year");
                                    ui.add_space(2.5);
                                    ui.label(format!("This year I pledge to {}", pledge)).on_hover_text("Your pledge for this year");
                                    ui.add_space(2.5);
                                    ui.collapsing("New years resolutions", |ui: &mut Ui| {
                                        for aspiration in asps {
                                            ui.label(aspiration.into_string().unwrap()).on_hover_text("You can do it!");
                                        }
                                    }).header_response.on_hover_text("Your resolutions for this year");
                                    ui.add_space(2.5);
                                }
                            }
                        }
                    });
                    ui.separator();
                    ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                        let year_folder = self.journal.entries.iter().find(|entry| entry.get_folder().unwrap().name == year.to_string()).expect("Failed to find displayed folder");
                        let months_folder = year_folder.get_folder().unwrap();
                        let current_month = horae::Utc::now().date().month;
                        for n in &months_folder.entries {
                            debug_assert!(n.is_folder());
                            let folder = n.get_folder().unwrap();
                            let name = month_num_to_name(folder.name.trim().parse().expect("Failed to parse month, month is not a number (u8)"));
                            let display_txt = {
                                if month_num_to_days(folder.name.parse().expect("Failed to parse month, month is not a number (u8)")) == folder.entries.len() as u8 {
                                    self.perfectly_completed(name)
                                } else if folder.name == current_month.to_string() {
                                    self.currently_selected(name)
                                } else {
                                    self.normally_completed(name)
                                }
                            };
                            ui.vertical_centered_justified(|ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    if ui.label(display_txt).on_hover_text("Click to open").clicked() {
                                        self.render.show_folder = ShowFolder::Month(year, folder.name.parse().expect("Failed to parse month, month is not a number (u8)"));
                                    }
                                });
                            });
                        }
                    });
                }
                ShowFolder::Month(year, month) => {
                    ui.vertical_centered_justified(|ui: &mut Ui| {
                        ui.add_space(22.5);
                            if ui.heading(format!("{} {}", month_num_to_name(month), year.to_string())).on_hover_text("Click to go back").clicked() {
                                self.go_back_one_level();
                            }
                        ui.add_space(22.5);
                    });
                    ui.separator();
                    ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                        let year_folder: &mut Folder = self.journal.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == year.to_string()).expect("Failed to find displayed folder").get_folder_mut().unwrap();
                        let month_folder: &mut Folder = year_folder.entries.iter_mut().find(|entry| entry.get_folder().unwrap().name == month.to_string()).expect("Failed to find displayed folder").get_folder_mut().unwrap();
                        for n in &mut month_folder.entries {
                            debug_assert!(n.is_journal_entry());
                            let entry = n.get_journal_entry().unwrap();
                            let body = {
                                let tmp = entry.text.clone();
                                let mut word_store = tmp.split_whitespace().take(25).collect::<Vec<&str>>();
                                if word_store.len() < tmp.split_whitespace().count() {
                                    word_store.push("...");
                                }
                                word_store.join(" ")
                            };
                            ui.vertical_centered_justified(|ui: &mut Ui| {
                                ui.group(|ui: &mut Ui| {
                                    if ui.label(entry.title.as_str()).on_hover_text("Date of the entry. Click to open").clicked() {
                                        self.journal.current_entry = entry.clone();
                                    };
                                    ui.group(|ui: &mut Ui| {
                                        if ui.label(body).on_hover_text("Click to open").clicked() {
                                            self.journal.current_entry = entry.clone();
                                        };
                                    });
                                });
                            });
                        };
                    });
                }
            };
        });
    }

    pub fn go_back_one_level(&mut self) {
        match &self.render.show_folder {
            ShowFolder::All => {}
            ShowFolder::Year(_) => self.render.show_folder = ShowFolder::All,
            ShowFolder::Month(year, _) => self.render.show_folder = ShowFolder::Year(*year),
        }
    }

    fn currently_selected(&self, name: &str) -> String {
        format!(
            "{} {} {}",
            self.settings.gui.file_marker_currently.start,
            name,
            self.settings.gui.file_marker_currently.end
        )
    }

    fn perfectly_completed(&self, name: &str) -> String {
        format!(
            "{} {} {}",
            self.settings.gui.file_marker_perfectly.start,
            name,
            self.settings.gui.file_marker_perfectly.end
        )
    }

    fn normally_completed(&self, name: &str) -> String {
        format!(
            "{} {} {}",
            self.settings.gui.file_marker_normally.start,
            name,
            self.settings.gui.file_marker_normally.end
        )
    }
}

pub fn month_num_to_name(month: u8) -> &'static str {
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
