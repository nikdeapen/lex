use crate::{Context, Token};

impl<'a> Context<'a> {
    //! Exact

    /// Parses an optional exact string `s`.
    ///
    /// Returns `(Some(s), after_s)`.
    /// Returns `(None, self)` if the parser does not start with `s`.
    /// Returns `(None, self)` if splitting on `s` breaks a `CRLF` sequence.
    pub fn exact(&self, s: &str) -> (Option<Token<'a>>, Self) {
        if self.value().starts_with(s) {
            if s.ends_with("\r")
                && self.len() > s.len()
                && self.value().as_bytes()[s.len()] == b'\n'
            {
                (None, *self)
            } else {
                let (left, right) = unsafe { self.split(s.len()) };
                (Some(left.token()), right)
            }
        } else {
            (None, *self)
        }
    }
}
