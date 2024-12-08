pub struct Settings {
    pub size: [f32; 2],
    pub side_panel_width: f32,
}

impl Default for Settings {
    // TODO: get from config
    // TODO: save to config
    // TODO: sensible defaults
    fn default() -> Self {
        Self {
            size: [800.0, 400.0],
            side_panel_width: 300.0,
        }
    }
}
