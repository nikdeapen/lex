use crate::{ParseContext, Token};

impl<'a> ParseContext<'a> {
    // Punctuation Marks

    /// Parses a punctuation mark.
    ///
    /// Returns `(Some(mark), after_mark)`.
    /// Returns `(None, self)` if the next token is not `mark`.
    pub fn mark(&self, mark: char) -> (Option<Token>, Self) {
        let mut s: [u8; 4] = [0u8; 4];
        let mark: &str = mark.encode_utf8(&mut s);
        self.exact(mark)
    }

    /// Parses a punctuation mark after optional white-line-comments.
    ///
    /// Returns `(Some(mark), after_mark)`.
    /// Returns `(None, self)` if the next non-white token is not `mark`.
    pub fn white_mark(&self, mark: char) -> (Option<Token>, Self) {
        self.white_line_comments();
        self.mark(mark)
    }
}
