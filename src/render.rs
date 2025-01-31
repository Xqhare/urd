use std::sync::{atomic::AtomicBool, Arc};

use crate::journal_entries::JournalEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShowFolder {
    All,
    Year(u16),
    /// year, month
    Month(u16, u8),
}

pub struct Render {
    pub view: View,
    pub show_folder: ShowFolder,
    pub entities: Entities,
    pub show_tips_and_tricks: bool,
    pub show_setup_wizard: bool,
    pub edit_all_aspirations: bool,
}

impl Render {
    pub fn startup_default() -> Self {
        Render {
            view: View::startup_default(),
            show_folder: ShowFolder::All,
            entities: Entities::default(),
            show_tips_and_tricks: true,
            show_setup_wizard: false,
            edit_all_aspirations: false,
        }
    }

    pub fn new(show_tips_and_tricks: bool) -> Self {
        Render {
            view: View::default(),
            show_folder: ShowFolder::All,
            entities: Entities::default(),
            show_tips_and_tricks,
            show_setup_wizard: false,
            edit_all_aspirations: false,
        }
    }
}

#[derive(Default)]
pub struct Entities {
    pub important_day_entries: Vec<JournalEntry>,
    pub mood_entries: Vec<JournalEntry>,
    pub aspirations: Vec<Aspirations>,
}

#[derive(Default)]
pub struct View {
    pub viewports: Viewports,
    pub pages: Pages,
    pub ui_state: UiState,
}

impl View {
    pub fn startup_default() -> Self {
        View {
            viewports: Viewports::startup_default(),
            pages: Pages::default(),
            ui_state: UiState::default(),
        }
    }
}

#[derive(Default)]
pub struct UiState {
    pub show_add_mood_field: bool,
    pub show_destructive_action_confirmation: bool,
}

pub struct Aspirations {
    pub year: String,
    pub edit_theme: String,
    pub edit_pledge: String,
    pub edit_resolutions: Vec<String>,
}

impl Default for Aspirations {
    fn default() -> Self {
        Aspirations {
            year: "".to_string(),
            edit_theme: "".to_string(),
            edit_pledge: "".to_string(),
            edit_resolutions: Vec::new(),
        }
    }
}

impl Aspirations {
    /// Takes in a year and returns a new Aspirations struct
    pub fn new(year: String) -> Self {
        Aspirations {
            year,
            edit_theme: "".to_string(),
            edit_pledge: "".to_string(),
            edit_resolutions: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct Pages {
    pub show_settings_page: bool,
    pub show_search_page: bool,
    pub show_file_picker_page: bool,
    pub show_important_days_page: bool,
    pub show_mood_page: bool,
}

pub struct Viewports {
    pub show_about_viewport: Arc<AtomicBool>,
    pub show_licenses_viewport: Arc<AtomicBool>,
    pub show_help_viewport: Arc<AtomicBool>,
}

impl Default for Viewports {
    fn default() -> Self {
        Viewports {
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
        }
    }
}

impl Viewports {
    pub fn startup_default() -> Self {
        Viewports {
            // ONLY DIFFERENCE
            show_help_viewport: Arc::new(AtomicBool::new(true)),
            // Same
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_about_viewport: Arc::new(AtomicBool::new(false)),
        }
    }
}
