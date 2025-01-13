use eframe::egui::{self, CentralPanel, ScrollArea, TextEdit, TopBottomPanel, Ui};

use super::UrdState;

impl UrdState {
    pub fn search_page(&mut self, ctx: &egui::Context) {
        self.search_top_panel(ctx);
        self.search_central_panel(ctx);
    }

    fn search_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("search_panel").show(ctx, |ui: &mut Ui| {
            ui.vertical_centered_justified(|ui: &mut Ui| {
                ui.add_space(1.0);
                ui.horizontal(|ui: &mut Ui| {
                    if ui.button("Back").on_hover_text("Go back to home").clicked() {
                        self.render.view.pages.show_search_page = false;
                    };
                    if ui.button("Clear").on_hover_text("Clear search query").clicked() {
                        self.search.query = "".to_string();
                        self.search.results = Vec::new();
                    }
                    if ui.button("Search").on_hover_text("Search for the query").clicked() {
                        self.search_current_query();
                    }
                    let t = ui.add(
                        TextEdit::singleline(&mut self.search.query)
                            .hint_text("Enter search query, separated by commas. Hit enter to search")
                            .desired_width(f32::INFINITY),
                    );
                    t.request_focus();
                    if t.changed() {
                        self.search_current_query();
                    }
                });
                ui.add_space(1.0);
            });
        });
    }

    pub fn search_current_query(&mut self) {
        let tokens = tokenize_search_query(&self.search.query);
        self.search.results = Vec::new();
        self.search_loop(tokens);
    }

    fn search_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                for result in &self.search.results {
                    ui.group(|ui: &mut Ui| {
                        ui.vertical_centered_justified(|ui: &mut Ui| {
                            let mut clicked = false;
                            if ui.heading(&result.title).on_hover_text("Click to open entry").clicked() {
                                clicked = true;
                            }
                            ui.separator();
                            if ui.label(&result.text).on_hover_text("Click to open entry").clicked() {
                                clicked = true;
                            }
                            if clicked {
                                self.journal.current_entry = result.clone();
                                self.render.view.pages.show_search_page = false;
                            }
                        });
                    });
                }
            })
        });
    }

    fn search_loop(&mut self, tokens: Vec<String>) {
        for year in &self.journal.entries {
            debug_assert!(year.is_folder());
            for month in &year.get_folder().unwrap().entries {
                debug_assert!(month.is_folder());
                for entry in &month.get_folder().unwrap().entries {
                    debug_assert!(entry.is_journal_entry());
                    for token in &tokens {
                        let journal_entry = entry.get_journal_entry().unwrap();
                        let token_found = {
                            let mut out = false;
                            if journal_entry.title.contains(token) {
                                out = true;
                            } else if journal_entry.text.contains(token) {
                                out = true;
                            } else {
                                let proj = journal_entry.metadata.get("projects");
                                if proj.is_some() {
                                    let project_ary = proj.unwrap().into_array();
                                    if project_ary.is_some() {
                                        for project in project_ary.unwrap() {
                                            if project.into_string().unwrap().contains(token) {
                                                out = true;
                                            }
                                        }
                                    }
                                }

                                let context = journal_entry.metadata.get("context_tags");
                                if context.is_some() {
                                    let context_ary = context.unwrap().into_array();
                                    if context_ary.is_some() {
                                        for context in context_ary.unwrap() {
                                            if context.into_string().unwrap().contains(token) {
                                                out = true;
                                            }
                                        }
                                    }
                                }

                                let special = journal_entry.metadata.get("special_tags");
                                if special.is_some() {
                                    let special_obj = special.unwrap().into_object();
                                    if special_obj.is_some() {
                                        for (key, value) in special_obj.unwrap().iter() {
                                            if key.contains(token)
                                                || value.into_string().unwrap().contains(token)
                                            {
                                                out = true;
                                            }
                                        }
                                    }
                                }
                            };
                            out
                        };
                        if token_found {
                            let entry_already_in_results = {
                                let mut out = false;
                                for t in &self.search.results {
                                    if token == &t.title {
                                        out = true;
                                    }
                                }
                                out
                            };
                            if entry_already_in_results {
                                continue;
                            } else {
                                self.search.results.push(journal_entry.clone());
                            }
                        }
                    }
                }
            }
        }
    }
}

fn tokenize_search_query(query: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for thing in query.trim().split(",") {
        if thing.len() != 0 {
            out.push(thing.trim().to_string());
        }
    }
    out
}
