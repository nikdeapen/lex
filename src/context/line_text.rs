use crate::ParseContext;

impl<'a> ParseContext<'a> {
    //! Line Text

    /// Gets the optional text for the 0-indexed `line_number`.
    ///
    /// Returns `None` when the `line_number` is invalid.
    pub fn get_line_text(&self, line_number: usize) -> Option<&'a str> {
        let mut c: ParseContext = *self;
        for _ in 0..line_number {
            if let (_l, Some(_le), after_le) = c.rest_of_line() {
                c = after_le;
            } else {
                return None;
            }
        }
        let (line, _, _) = c.rest_of_line();
        Some(line.value())
    }
}
