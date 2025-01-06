
use eframe::egui::{CentralPanel, Context, Ui};

use crate::journal_entries::JournalEntry;

use super::UrdState;

impl UrdState {
    pub fn important_days_page(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Important Days");
            for tmp in &self.render.entities.important_day_entries {
                ui.label(tmp.title.clone());
                ui.separator();
            }
        });
    }

    pub fn construct_important_day_entries(&mut self) {
        self.render.entities.important_day_entries.clear();
        for year in &self.journal.entries {
            debug_assert!(year.is_folder());
            for month in &year.get_folder().unwrap().entries {
                debug_assert!(month.is_folder());
                for entry in &month.get_folder().unwrap().entries {
                    debug_assert!(entry.is_journal_entry());
                    let entry = entry.get_journal_entry().unwrap();
                    if entry.metadata.get("important_day").unwrap().into_boolean().unwrap() {
                        self.render.entities.important_day_entries.push(entry.clone());
                    }
                }
            }
        }
    }
}
