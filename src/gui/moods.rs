
use eframe::egui::{CentralPanel, Color32, Context, Grid, Rounding, ScrollArea, Ui};

use super::{main_page_side_panel::month_num_to_name, UrdState};

impl UrdState {
    pub fn moods_page(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.horizontal_wrapped(|ui: &mut Ui| {
                for (mood, colour) in self.journal.moods.iter() {
                    let (r, g, b, a) = {
                        let ary = colour.into_array().unwrap().into_vec();
                        (
                        ary[0].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                        ary[1].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                        ary[2].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                        ary[3].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                        )
                    };
                    let mood_colour = Color32::from_rgba_unmultiplied(r, g, b, a);
                    ui.colored_label(mood_colour, mood);
                }
            });
            ui.vertical_centered_justified(|ui: &mut Ui| {
                ui.separator();
                ui.heading("Your mood over time");
                ui.separator();
            });
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                for year in &self.journal.entries {
                    ui.group(|ui: &mut Ui| {
                        ui.vertical_centered_justified(|ui: &mut Ui| {
                            ui.heading(format!("{}", year.get_folder().unwrap().name));
                            ui.label("31. <-- | --> 01.");
                        });
                        for month in &year.get_folder().unwrap().entries {
                            ui.horizontal(|ui: &mut Ui| {
                                ui.label(format!("{}", month_num_to_name(month.get_folder().unwrap().name.parse().expect("Failed to parse month, month is not a number (u8)"))));
                                for day in &month.get_folder().unwrap().entries {
                                    let entry = day.get_journal_entry().unwrap();
                                    let mood = entry.metadata.get("mood").unwrap().into_string().unwrap();
                                    let (r, g, b, a) = {
                                        let tmp = self.journal.moods.get(&mood).unwrap();
                                        let ary = tmp.into_array().unwrap().into_vec();
                                        (
                                        ary[0].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                                        ary[1].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                                        ary[2].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                                        ary[3].into_number().unwrap().into_usize().unwrap().try_into().expect("Colour value out of range"),
                                        )
                                    };
                                    let mood_colour = Color32::from_rgba_unmultiplied(r, g, b, a);

                                    ui.group(|ui: &mut Ui| {
                                        ui.label("     ");
                                        let area = ui.min_rect();
                                        ui.painter().rect_filled( area , 2.0, mood_colour);
                                    });
                                };
                            });
                        };
                    });
                };
            });
        });
    }

    pub fn construct_mood_entries(&mut self) {
        self.render.mood_entries.clear();
        for year in &self.journal.entries {
            debug_assert!(year.is_folder());
            for month in &year.get_folder().unwrap().entries {
                debug_assert!(month.is_folder());
                for entry in &month.get_folder().unwrap().entries {
                    debug_assert!(entry.is_journal_entry());
                    let entry = entry.get_journal_entry().unwrap();
                    if entry.metadata.get("mood").unwrap().into_string().unwrap() != "" {
                        self.render.mood_entries.push(entry.clone());
                    }
                }
            }
        }
    }
}
