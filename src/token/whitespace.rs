use crate::Token;

impl<'a> Token<'a> {
    //! Line Endings

    /// Gets the length of the line-ending from the prefix of `s`.
    ///
    /// Returns `2` if `s` starts with `CRLF`.
    /// Returns `1` if `s` starts with `CR` and not `CRLF`.
    /// Returns `1` if `s` starts with `LF`.
    /// Returns `0` if `s` does not start with `CR` or `LF`.
    pub(crate) fn line_ending_prefix_len(s: &str) -> usize {
        if s.is_empty() {
            0
        } else if s.as_bytes()[0] == b'\r' {
            if s.len() >= 2 && s.as_bytes()[1] == b'\n' {
                2
            } else {
                1
            }
        } else if s.as_bytes()[0] == b'\n' {
            1
        } else {
            0
        }
    }

    /// Counts the number of line-endings and gets the number of bytes after the last line-ending.
    ///
    /// A line-ending is a `CR`, `LF`, or `CRLF` sequence.
    /// The `CRLF` sequence is always treated as a single line-ending.
    ///
    /// Returns `(line_ending_count, after_last_line_ending_len)`.
    /// Returns `(0, self.len())` if there are no line-endings.
    pub(crate) fn line_ending_count_and_last_line_len(&self) -> (usize, usize) {
        let mut s: &str = self.value();
        let mut line_ending_count: usize = 0;
        while let Some(cr_or_lf) = s.as_bytes().iter().position(|c| *c == b'\r' || *c == b'\n') {
            s = &s[cr_or_lf..];
            s = &s[Self::line_ending_prefix_len(s)..];
            line_ending_count += 1;
        }
        (line_ending_count, s.len())
    }
}
