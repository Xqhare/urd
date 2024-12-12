
use eframe::{egui, Frame};

use super::UrdState;

impl UrdState {
    pub fn search_page(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        self.search_top_panel(ctx, frame);
        self.search_central_panel(ctx, frame);
    }

    fn search_top_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
    }

    fn search_central_panel(&mut self, ctx: &egui::Context, frame: &mut Frame) {
    }
}
