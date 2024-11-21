use std::fmt::Debug;

use crate::{ParseContext, Token};

impl<'a> ParseContext<'a> {
    //! Punctuation

    /// Parses the punctuation char `c`. (ignores any white-line-comments prefix)
    ///
    /// Returns `Ok(c, after_c)`.
    pub fn parse_char<E>(self, c: char, error: E) -> crate::Result<'a, Token<'a>, E>
    where
        E: Debug,
    {
        let (_white, after_white) = self.white_line_comments();
        let mut b: [u8; 4] = [0u8; 4];
        let c: &str = c.encode_utf8(&mut b);
        if let (Some(c), after_c) = after_white.exact(c) {
            Ok((c, after_c))
        } else {
            Err(after_white.to_error(error))
        }
    }
}
