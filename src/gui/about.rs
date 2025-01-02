use eframe::egui::{CentralPanel, Context, Ui, ViewportBuilder, ViewportId};

use super::UrdState;

const ABOUT_WINDOW_SIZE: [f32; 2] = [200.0, 130.0];

impl UrdState {
    pub fn about_viewport_startup(&mut self, ctx: &Context) {
        if self
            .render
            .viewports
            .show_about_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let show_viewport_pointer = self.render.viewports.show_about_viewport.clone();
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("about_viewport"),
                ViewportBuilder::default()
                    .with_title("About")
                    .with_inner_size(ABOUT_WINDOW_SIZE)
                    .with_min_inner_size(ABOUT_WINDOW_SIZE)
                    .with_max_inner_size(ABOUT_WINDOW_SIZE),
                move |ctx, class| {
                    assert!(class == eframe::egui::ViewportClass::Deferred);
                    CentralPanel::default().show(ctx, |ui: &mut Ui| {
                        ui.horizontal_wrapped(|ui: &mut Ui| {
                            ui.vertical_centered_justified(|ui: &mut Ui| {
                                ui.add_space(10.0);
                                ui.label(format!("Written by {}", env!("CARGO_PKG_AUTHORS")));
                                ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                                ui.hyperlink_to("Urd on GitHub", "https://github.com/xqhare/urd");
                                ui.separator();
                                ui.label("In loving memory of my stepfather.");
                            });
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
