use std::sync::{atomic::AtomicBool, Arc};

pub enum ShowFolder {
    All,
    Year(u16),
    /// year, month
    Month(u16, u8),
}

pub struct Render {
    pub viewports: Viewport,
    pub show_folder: ShowFolder,
}

impl Default for Render {
    fn default() -> Self {
        Render {
            viewports: Viewport::default(),
            show_folder: ShowFolder::All,
        }
    }
}

pub struct Viewport {
    pub show_about_viewport: Arc<AtomicBool>,
    pub show_licenses_viewport: Arc<AtomicBool>,
    pub show_help_viewport: Arc<AtomicBool>,
    pub show_settings_viewport: bool,
    pub show_search_page: bool,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            show_about_viewport: Arc::new(AtomicBool::new(false)),
            show_licenses_viewport: Arc::new(AtomicBool::new(false)),
            show_help_viewport: Arc::new(AtomicBool::new(false)),
            show_settings_viewport: false,
            show_search_page: false,
        }
    }
}
