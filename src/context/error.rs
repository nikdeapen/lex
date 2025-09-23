use std::fmt::Debug;
use crate::{Error, ParseContext};

impl<'a> ParseContext<'a> {
    //! Error

    /// Converts the parse context to an error.
    pub fn to_error<E>(self, e: E) -> Error<'a, E>
    where
        E: Debug,
    {
        self.token().to_error(self.config(), e)
    }
}

impl<'a> ParseContext<'a> {
    //! Line Text

    /// Gets the optional text for the 0-indexed `line_number`.
    ///
    /// Returns `None` when the `line_number` is invalid.
    ///
    /// # Note
    /// This is not efficient and scans the entire context.
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
