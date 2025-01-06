use eframe::egui::{CentralPanel, Context, ScrollArea, Ui};

use super::UrdState;

impl UrdState {
    pub fn important_days_page(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                ui.vertical_centered_justified(|ui: &mut Ui| {
                    ui.heading("All your important days");
                    ui.label("Click on an entry to open it.");
                    for tmp in &self.render.entities.important_day_entries {
                        ui.group(|ui: &mut Ui| {
                            let title = ui.label(tmp.title.clone());
                            let mood = ui.label(format!(
                                "Mood: {}",
                                tmp.metadata.get("mood").unwrap().into_string().unwrap()
                            ));
                            let sep = ui.separator();
                            let text = ui.label(tmp.text.clone());
                            if title.clicked() || mood.clicked() || sep.clicked() || text.clicked()
                            {
                                self.journal.current_entry = tmp.clone();
                                self.render.view.pages.show_important_days_page = false;
                            }
                        });
                    }
                })
            })
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
                    if entry
                        .metadata
                        .get("important_day")
                        .unwrap()
                        .into_boolean()
                        .unwrap()
                    {
                        self.render
                            .entities
                            .important_day_entries
                            .push(entry.clone());
                    }
                }
            }
        }
    }
}
