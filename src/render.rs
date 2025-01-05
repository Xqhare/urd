use std::sync::{atomic::AtomicBool, Arc};

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
    pub show_add_mood_field: bool,
}

impl Default for Render {
    fn default() -> Self {
        Render {
            view: View::default(),
            show_folder: ShowFolder::All,
            show_add_mood_field: false,
        }
    }
}

impl Render {
    pub fn startup_default() -> Self {
        Render {
            view: View::startup_default(),
            show_folder: ShowFolder::All,
            show_add_mood_field: false,
        }
    }
}

pub struct View {
    pub show_about_viewport: Arc<AtomicBool>,
    pub show_licenses_viewport: Arc<AtomicBool>,
    pub show_help_viewport: Arc<AtomicBool>,
    pub show_settings_viewport: bool,
    pub show_search_page: bool,
    pub show_file_picker: bool,
    pub show_important_days_page: bool,
    pub show_mood_page: bool,
}

impl Default for View {
    fn default() -> Self {
        View {
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
            show_settings_viewport: false,
            show_search_page: false,
            show_file_picker: false,
            show_important_days_page: false,
            show_mood_page: false,
        }
    }
}

impl View {
    pub fn startup_default() -> Self {
        View {
            // ONLY DIFFERENCE
            show_help_viewport: Arc::new(AtomicBool::new(true)),
            // Same
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_settings_viewport: false,
            show_search_page: false,
            show_file_picker: false,
            show_important_days_page: false,
            show_mood_page: false,
        }
    }
}
