
pub struct Error {
    pub show_error: bool,
    pub error_message: Option<String>,
}

impl Error {
    /// Create a new error with a message to be displayed and set `show_error` to true
    ///
    /// Use `Default::default` to create a blank error with `show_error` set to false
    pub fn new<S: Into<String>>(msg: S) -> Self {
        Self {
            show_error: true,
            error_message: Some(msg.into()),
        }
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            show_error: false,
            error_message: None,
        }
    }
}
