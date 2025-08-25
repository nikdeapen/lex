/// A parse config.
#[derive(Clone, Debug)]
pub struct Config {
    line_comment_delimiter: Option<String>,
    spaces_per_tab: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            line_comment_delimiter: None,
            spaces_per_tab: 4,
        }
    }
}

impl Config {
    //! Line Comment Delimiter

    /// Checks if the `line_comment_delimiter` is valid.
    pub fn is_valid_line_comment_delimiter(line_comment_delimiter: &str) -> bool {
        !line_comment_delimiter.is_empty()
            && !line_comment_delimiter.contains("\r")
            && !line_comment_delimiter.contains("\n")
    }

    /// Gets the optional line-comment delimiter.
    pub fn line_comment_delimiter(&self) -> Option<&str> {
        self.line_comment_delimiter.as_deref()
    }

    /// Sets the `line_comment_delimiter`.
    ///
    /// # Safety
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

    /// Sets the `line_comment_delimiter`.
    ///
    /// # Safety
    /// The `line_comment_delimiter` must be valid.
    pub unsafe fn with_line_comment_delimiter<S>(mut self, line_comment_delimiter: S) -> Self
    where
        S: Into<String>,
    {
        self.set_line_comment_delimiter(line_comment_delimiter);
        self
    }
}

impl Config {
    //! Spaces Per Tab

    /// Gets the spaces per tab.
    pub fn spaces_per_tab(&self) -> usize {
        self.spaces_per_tab
    }

    /// Sets the `spaces_per_tab`.
    ///
    /// # Safety
    /// The `spaces_per_tab` cannot be `0`.
    pub unsafe fn set_spaces_per_tab(&mut self, spaces_per_tab: usize) {
        debug_assert_ne!(spaces_per_tab, 0);

        self.spaces_per_tab = spaces_per_tab;
    }

    /// Sets the `spaces_per_tab`.
    ///
    /// # Safety
    /// The `spaces_per_tab` cannot be `0`.
    pub unsafe fn with_spaces_per_tab(mut self, spaces_per_tab: usize) -> Self {
        debug_assert_ne!(spaces_per_tab, 0);

        self.set_spaces_per_tab(spaces_per_tab);
        self
    }
}
