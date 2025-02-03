use crate::ParseContext;

impl<'a> ParseContext<'a> {
    //! Split

    /// Splits the parser at the `index`.
    ///
    /// Returns `(before_index, index_and_after)`.
    ///
    /// # Unsafe,
    /// The `index` must be a valid split index.
    pub unsafe fn split(&self, index: usize) -> (Self, Self) {
        debug_assert!(self.token().is_valid_split_index(index));

        let (left, right) = self.token().split(index);
        (
            Self::new(left, self.config()),
            Self::new(right, self.config()),
        )
    }

    /// Splits the parser at the `index`.
    ///
    /// Returns `(Some(before_index), index_and_after)` if the `index` is not `0`.
    /// Returns `(None, self)` if the `index` is `0`.
    ///
    /// # Unsafe,
    /// The `index` must be a valid split index.
    pub unsafe fn split_optional(&self, index: usize) -> (Option<Self>, Self) {
        debug_assert!(self.token().is_valid_split_index(index));

        if index == 0 {
            (None, *self)
        } else {
            let (left, right) = self.split(index);
            (Some(left), right)
        }
    }
}
