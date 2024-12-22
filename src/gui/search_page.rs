
use eframe::{egui::{self, CentralPanel, TopBottomPanel, Ui}, Frame};

use super::UrdState;

impl UrdState {
    pub fn search_page(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.search_top_panel(ctx, frame);
        self.search_central_panel(ctx, frame);
    }

    fn search_top_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        TopBottomPanel::top("search_panel").show(ctx, |ui: &mut Ui| {
            ui.horizontal(|ui: &mut Ui| {
                if ui.button("Back").clicked() {
                    self.render.viewports.show_search_page = false;
                };
                ui.label("Search");
            });
        });
    }

    fn search_central_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.label("Search results");
        });
    }
}
