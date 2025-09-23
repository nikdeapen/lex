use crate::Token;
use std::fmt::Debug;

/// A parsing error.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Error<'a, E: Debug> {
    token: Token<'a>,
    error: E,
}

impl<'a, E: Debug> Error<'a, E> {
    //! Construction

    /// Creates a new parsing error.
    pub fn new(token: Token<'a>, error: E) -> Self {
        Self { token, error }
    }
}

impl<'a, E: Debug> Error<'a, E> {
    //! Properties

    /// Gets the token.
    pub fn token(&self) -> Token<'a> {
        self.token
    }

    /// Gets the typed error.
    pub fn error(&self) -> &E {
        &self.error
    }

    /// Converts the error to the typed error.
    pub fn to_error(self) -> E {
        self.error
    }
}

impl<'a, E: Debug> Error<'a, E> {
    //! Map

    /// Maps the typed error.
    pub fn map<F, E2>(self, map_fn: F) -> Error<'a, E2>
    where
        F: Fn(E) -> E2,
        E2: Debug,
    {
        let token: Token = self.token;
        let error: E2 = map_fn(self.error);
        Error { token, error }
    }
}
