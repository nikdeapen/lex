use crate::{Config, Token};

/// A lexical token with an associated config.
#[derive(Copy, Clone, Debug)]
pub struct ParseContext<'a> {
    token: Token<'a>,
    config: &'a Config,
}

impl<'a> ParseContext<'a> {
    //! Construction

    /// Creates a new parse context.
    pub fn new(token: Token<'a>, config: &'a Config) -> Self {
        Self { token, config }
    }
}

impl<'a> ParseContext<'a> {
    //! Token Properties

    /// Gets the token.
    pub fn token(self) -> Token<'a> {
        self.token
    }

    /// Gets the token value.
    pub fn value(self) -> &'a str {
        self.token().value()
    }

    /// Gets the length of the token.
    pub fn len(&self) -> usize {
        self.token.len()
    }

    /// Checks if the token is empty.
    pub fn is_empty(&self) -> bool {
        self.token.is_empty()
    }
}

impl<'a> ParseContext<'a> {
    //! Config Properties

    /// Gets the config.
    pub fn config(&self) -> &'a Config {
        self.config
    }
}
