use crate::ParseContext;

impl<'a> ParseContext<'a> {
    //! Match Prefix

    /// Gets the number of bytes from the prefix that match `prefix_fn`.
    fn match_prefix_len<F>(&self, prefix_fn: F) -> usize
    where
        F: Fn(u8) -> bool,
    {
        self.value()
            .as_bytes()
            .iter()
            .position(|c| !prefix_fn(*c))
            .unwrap_or(self.len())
    }

    /// Parses the number of bytes from the prefix that match `prefix_fn`.
    ///
    /// Returns `(Some(matched_prefix), after_matched_prefix)`.
    /// Returns `(None, self)` when no bytes match the prefix.
    ///
    /// # Unsafe
    /// The `prefix_fn` must result in a valid split index. This can be ensured if the `prefix_fn`
    /// matches only US-ASCII chars and not line-endings.
    pub unsafe fn match_prefix_optional_unchecked<F>(&self, prefix_fn: F) -> (Option<Self>, Self)
    where
        F: Fn(u8) -> bool,
    {
        self.split_optional_unchecked(self.match_prefix_len(prefix_fn))
    }

    /// Parses the number of bytes from the prefix that match `prefix_fn`.
    ///
    /// Returns `(matched_prefix, after_matched_prefix)`.
    /// Returns `(empty, self)` when no bytes match the prefix.
    ///
    /// # Unsafe
    /// The `prefix_fn` must result in a valid split index. This can be ensured if the `prefix_fn`
    /// matches only US-ASCII chars and not line-endings
    pub unsafe fn match_prefix_unchecked<F>(&self, prefix_fn: F) -> (Self, Self)
    where
        F: Fn(u8) -> bool,
    {
        self.split_unchecked(self.match_prefix_len(prefix_fn))
    }
}