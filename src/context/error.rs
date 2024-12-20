use std::fmt::Debug;

use crate::{Error, ParseContext};

impl<'a> ParseContext<'a> {
    //! Error

    /// Converts the parser to an error.
    pub fn to_error<E>(self, e: E) -> Error<'a, E>
    where
        E: Debug,
    {
        self.token().to_error(self.config(), e)
    }
}
