use crate::Context;

impl<'a> Context<'a> {
    //! Rest of Line

    /// Parses the rest of the line.
    ///
    /// Returns `(rest_of_line, Some(line_ending), after_line_ending)`.
    /// Returns `(rest_of_lex, None, empty)` when there is no line-ending.
    pub fn rest_of_line(&self) -> (Self, Option<Self>, Self) {
        let cr_lf_or_end = self
            .value()
            .as_bytes()
            .iter()
            .position(|c| *c == b'\r' || *c == b'\n')
            .unwrap_or(self.len());
        let (rest_of_line, after_rest_of_line) = unsafe { self.split(cr_lf_or_end) };
        let (line_ending, after_line_ending) = after_rest_of_line.line_ending();
        (rest_of_line, line_ending, after_line_ending)
    }
}
