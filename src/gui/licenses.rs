use eframe::egui::{CentralPanel, Context, ScrollArea, Ui, ViewportBuilder, ViewportId};

use super::UrdState;

const LICENSES_WINDOW_SIZE: [f32; 2] = [300.0, 500.0];
const EFRAME_LICENSE: &str = include_str!("../../licenses/MIT-Eframe");
const HORAE_LICENSE: &str = include_str!("../../licenses/MIT-Horae");
const NABU_LICENSE: &str = include_str!("../../licenses/MIT-Nabu");

impl UrdState {
    pub fn licenses_viewport_startup(&mut self, ctx: &Context) {
        if self
            .render
            .view
            .viewports
            .show_licenses_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let show_viewport_pointer = self.render.view.viewports.show_licenses_viewport.clone();
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("licenses_viewport"),
                ViewportBuilder::default()
                    .with_title("Licenses")
                    .with_inner_size(LICENSES_WINDOW_SIZE)
                    .with_min_inner_size(LICENSES_WINDOW_SIZE)
                    .with_max_inner_size(LICENSES_WINDOW_SIZE),
                move |ctx, class| {
                    assert!(class == eframe::egui::ViewportClass::Deferred);

                    CentralPanel::default().show(ctx, |ui: &mut Ui| {
                        ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                            ui.heading("Licenses");
                            ui.separator();
                            ui.hyperlink_to("eframe", "https://github.com/emilk/egui");
                            ui.label("MIT License");
                            ui.label(EFRAME_LICENSE);
                            ui.separator();
                            ui.hyperlink_to("Horae", "https://github.com/xqhare/horae");
                            ui.label("MIT License");
                            ui.label(HORAE_LICENSE);
                            ui.separator();
                            ui.hyperlink_to("Nabu", "https://github.com/xqhare/nabu");
                            ui.label("MIT License");
                            ui.label(NABU_LICENSE);
                        });
                    });
                    // Close viewport
                    if ctx.input(|i| i.viewport().close_requested()) {
                        show_viewport_pointer.store(false, std::sync::atomic::Ordering::Relaxed);
                    }
                },
            );
        }
    }
}
