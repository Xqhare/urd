
use eframe::egui::{CentralPanel, Context, Ui};

use super::UrdState;

impl UrdState {
    pub fn important_days_page(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.heading("Important Days");
        });
    }
}
