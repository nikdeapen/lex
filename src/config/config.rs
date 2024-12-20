use crate::CommentConfig;

/// A parsing config.
#[derive(Clone, Debug)]
pub struct Config {
    comment_config: CommentConfig,
    spaces_per_tab: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            comment_config: CommentConfig::default(),
            spaces_per_tab: 4,
        }
    }
}

impl Config {
    //! Comments

    /// Gets the comment config.
    pub fn comment_config(&self) -> &CommentConfig {
        &self.comment_config
    }

    /// Sets the comment config.
    pub fn set_comment_config(&mut self, comment_config: CommentConfig) {
        self.comment_config = comment_config
    }

    /// Sets the comment config.
    pub fn with_comment_config(mut self, comment_config: CommentConfig) -> Self {
        self.set_comment_config(comment_config);
        self
    }
}

impl Config {
    //! Spaces Per Tab

    /// Gets the spaces per tab.
    pub fn spaces_per_tab(&self) -> usize {
        self.spaces_per_tab
    }

    /// Sets the spaces per tab.
    ///
    /// # Unsafe
    /// The `spaces_per_tab` cannot be `0`.
    pub unsafe fn set_spaces_per_tab(&mut self, spaces_per_tab: usize) {
        debug_assert_ne!(spaces_per_tab, 0);

        self.spaces_per_tab = spaces_per_tab;
    }

    /// Sets the spaces per tab.
    ///
    /// # Unsafe
    /// The `spaces_per_tab` cannot be `0`.
    pub unsafe fn with_spaces_per_tab(mut self, spaces_per_tab: usize) -> Self {
        debug_assert_ne!(spaces_per_tab, 0);

        self.set_spaces_per_tab(spaces_per_tab);
        self
    }
}
