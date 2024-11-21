use std::fmt::Debug;

use crate::{ParseContext, Token};

impl<'a> ParseContext<'a> {
    //! Punctuation

    /// Parses the punctuation char `c`. (ignores any white-line-comments prefix)
    ///
    /// Returns `Ok(c, after_c)`.
    pub fn parse_char<E, F>(&self, c: char, error: F) -> crate::Result<Token, E>
    where
        E: Debug,
        F: Fn() -> E,
    {
        let (_white, after_white) = self.white_line_comments();
        let mut b: [u8; 4] = [0u8; 4];
        let c: &str = c.encode_utf8(&mut b);
        if let (Some(c), after_c) = after_white.exact(c) {
            Ok((c, after_c))
        } else {
            Err(after_white.to_error(error()))
        }
    }
}
