#[derive(Clone, Debug)]
#[derive(Default)]
pub struct Error {
    pub show_error: bool,
    pub error_context: Option<String>,
    pub error_message: Option<String>,
}

impl Error {
    /// Create a new error with a message to be displayed and set `show_error` to true
    ///
    /// param msg: The error message to be displayed
    /// param msg_ctx: The context of the error
    ///
    /// Msg_ctx is shown first, and is supposed to give the error needed context.
    /// Example:
    ///
    /// Instead of only showing "Permission denied", show "Journal cant be read: Permission
    /// denied".
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

