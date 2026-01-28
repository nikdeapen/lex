use crate::Context;

impl<'a> Context<'a> {
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
    /// # Safety
    /// The `prefix_fn` must result in a valid split index.
    pub unsafe fn match_prefix<F>(&self, prefix_fn: F) -> (Option<Self>, Self)
    where
        F: Fn(u8) -> bool,
    {
        self.split_optional(self.match_prefix_len(prefix_fn))
    }
}
