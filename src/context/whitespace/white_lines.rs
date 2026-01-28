use crate::Context;

impl<'a> Context<'a> {
    //! White Lines

    /// Parses optional non-empty whitespace & line-endings.
    ///
    /// Returns `(Some(line_ending), after_line_ending)`.
    /// Returns `(None, self)` when there is no line-ending.
    pub fn white_lines(&self) -> (Option<Self>, Self) {
        unsafe { self.match_prefix(|c| c == b' ' || c == b'\t' || c == b'\r' || c == b'\n') }
    }
}
