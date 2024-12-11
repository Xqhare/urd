use eframe::egui::{CentralPanel, Context, ScrollArea, Ui, ViewportBuilder, ViewportId};

use super::UrdState;

const LICENSES_WINDOW_SIZE: [f32; 2] = [300.0, 500.0];
const EFRAME_LICENSE: &str = "Copyright (c) 2018-2021 Emil Ernerfeldt <emil.ernerfeldt@gmail.com>

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the \"Software\"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.";

const HORAE_LICENSE: &str = "";

impl UrdState {
    pub fn licenses_viewport_startup(&mut self, ctx: &Context) {
        if self
            .show_licenses_viewport
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let show_viewport_pointer = self.show_licenses_viewport.clone();
            ctx.show_viewport_deferred(
                ViewportId::from_hash_of("licenses_viewport"),
                ViewportBuilder::default()
                    .with_title("Licenses")
                    .with_inner_size(LICENSES_WINDOW_SIZE)
                    .with_min_inner_size(LICENSES_WINDOW_SIZE)
                    .with_max_inner_size(LICENSES_WINDOW_SIZE),
                move |ctx, class| {
                    // TODO: paint the settings in the main window if this fails instead of
                    // panicking
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
