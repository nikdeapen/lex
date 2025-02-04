/// A comment parsing config.
#[derive(Clone, Debug, Default)]
pub struct CommentConfig {
    line_comment_delimiter: Option<String>,
}

impl CommentConfig {
    //! Line Comment Delimiter

    /// Checks if the line-comment delimiter is valid.
    pub fn is_valid_line_comment_delimiter(line_comment_delimiter: &str) -> bool {
        !line_comment_delimiter.is_empty()
            && !line_comment_delimiter.contains("\r")
            && !line_comment_delimiter.contains("\n")
    }

    /// Gets the optional line-comment delimiter.
    pub fn line_comment_delimiter(&self) -> Option<&str> {
        self.line_comment_delimiter.as_ref().map(|s| s.as_str())
    }

    /// Sets the line-comment delimiter.
    ///
    /// # Unsafe
    /// The `line_comment_delimiter` must be valid.
    pub unsafe fn set_line_comment_delimiter<S>(&mut self, line_comment_delimiter: S)
    where
        S: Into<String>,
    {
        let line_comment_delimiter: String = line_comment_delimiter.into();

        debug_assert!(Self::is_valid_line_comment_delimiter(
            line_comment_delimiter.as_str()
        ));

        self.line_comment_delimiter = Some(line_comment_delimiter.to_string());
    }

    /// Sets the line-comment delimiter.
    ///
    /// # Unsafe
    /// The `line_comment_delimiter` must be valid.
    pub unsafe fn with_line_comment_delimiter<S>(mut self, line_comment_delimiter: S) -> Self
    where
        S: Into<String>,
    {
        self.set_line_comment_delimiter(line_comment_delimiter);
        self
    }
}
