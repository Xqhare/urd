
#[derive(Clone, Debug)]
pub struct Error {
    pub show_error: bool,
    pub error_context: Option<String>,
    pub error_message: Option<String>,
}

impl Error {
    /// Create a new error with a message to be displayed and set `show_error` to true
    ///
    /// Use `Default::default` to create a blank error with `show_error` set to false
    pub fn new<S: Into<String>>(msg: S, msg_ctx: S) -> Self {
        Self {
            show_error: true,
            error_message: Some(msg.into()),
            error_context: Some(msg_ctx.into()),
        }
    }
}

impl Default for Error {
    fn default() -> Self {
        Self {
            show_error: false,
            error_message: None,
            error_context: None,
        }
    }
}
