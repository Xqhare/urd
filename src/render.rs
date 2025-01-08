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
}

impl Default for Render {
    fn default() -> Self {
        Render {
            view: View::default(),
            show_folder: ShowFolder::All,
            entities: Entities::default(),
        }
    }
}

impl Render {
    pub fn startup_default() -> Self {
        Render {
            view: View::startup_default(),
            show_folder: ShowFolder::All,
            entities: Entities::default(),
        }
    }
}

pub struct Entities {
    pub important_day_entries: Vec<JournalEntry>,
    pub mood_entries: Vec<JournalEntry>,
    pub aspirations: Vec<Aspirations>,
}

impl Default for Entities {
    fn default() -> Self {
        Entities {
            important_day_entries: Vec::new(),
            mood_entries: Vec::new(),
            aspirations: Vec::new(),
        }
    }
}

pub struct View {
    pub viewports: Viewports,
    pub pages: Pages,
    pub ui_state: UiState,
}

impl Default for View {
    fn default() -> Self {
        View {
            viewports: Viewports::default(),
            pages: Pages::default(),
            ui_state: UiState::default(),
        }
    }
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

pub struct UiState {
    pub show_add_mood_field: bool,
    pub show_destructive_action_confirmation: bool,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            show_add_mood_field: false,
            show_destructive_action_confirmation: false,
        }
    }
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

pub struct Pages {
    pub show_settings_page: bool,
    pub show_search_page: bool,
    pub show_file_picker_page: bool,
    pub show_important_days_page: bool,
    pub show_mood_page: bool,
}

impl Default for Pages {
    fn default() -> Self {
        Pages {
            show_settings_page: false,
            show_search_page: false,
            show_file_picker_page: false,
            show_important_days_page: false,
            show_mood_page: false,
        }
    }
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
