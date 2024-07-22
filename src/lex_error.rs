use crate::Token;

/// An error parsing a value.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct LexError<'a, E> {
    token: Token<'a>,
    error: E,
}

impl<'a, E> LexError<'a, E> {
    //! Construction

    /// Creates a new lex error.
    pub const fn new(token: Token<'a>, error: E) -> Self {
        Self { token, error }
    }
}

impl<'a, E> From<(Token<'a>, E)> for LexError<'a, E> {
    fn from(t: (Token<'a>, E)) -> Self {
        Self::new(t.0, t.1)
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

    /// Gets the error.
    pub fn to_error(self) -> E {
        self.error
    }
}
