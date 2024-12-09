use eframe::egui::{CentralPanel, Context, Ui, ViewportBuilder, ViewportId};

use super::UrdState;

impl UrdState {
    pub fn licenses_viewport_startup(&mut self, ctx: &Context) {
        if self.show_licenses_viewport.load(std::sync::atomic::Ordering::Relaxed) {
            let show_viewport_pointer = self.show_licenses_viewport.clone();
            ctx.show_viewport_deferred(ViewportId::from_hash_of("licenses_viewport"), ViewportBuilder::default().with_title("Licenses").with_inner_size([200.0, 400.0]), move |ctx, class| {
                // TODO: paint the settings in the main window if this fails instead of
                // panicking
                assert!(class == eframe::egui::ViewportClass::Deferred);
                CentralPanel::default().show(ctx, |ui: &mut Ui| {
                    ui.heading("Licenses");
                });
                // Close viewport
                if ctx.input(|i| i.viewport().close_requested()) {
                    show_viewport_pointer.store(false, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    }
}


