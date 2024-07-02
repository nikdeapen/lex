use crate::Token;

/// An error parsing a value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct LexError<'a, E> {
    token: Token<'a>,
    error: E,
}

impl<'a, E> From<(Token<'a>, E)> for LexError<'a, E> {
    fn from(tuple: (Token<'a>, E)) -> Self {
        Self {
            token: tuple.0,
            error: tuple.1,
        }
    }
}

impl<'a> From<Token<'a>> for LexError<'a, ()> {
    fn from(token: Token<'a>) -> Self {
        Self { token, error: () }
    }
}

impl<'a, E> LexError<'a, E> {
    //! Properties

    /// Gets the token.
    pub fn token(&self) -> Token<'a> {
        self.token
    }

    /// Gets the error.
    pub fn error(&self) -> &E {
        &self.error
    }
}
