
use eframe::egui::{CentralPanel, Context, Ui};

use super::UrdState;

impl UrdState {
    pub fn moods_page(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Important Days");
        });
    }
}
