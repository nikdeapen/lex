use crate::lexer::{Span, Token, TokenKind};
use std::fmt::{Display, Formatter};

/// A parse error. (an unexpected token)
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ParseError<K> {
    expected: K,
    found: Token<K>,
}

impl<K> ParseError<K> {
    //! Construction

    /// Creates a new parse error.
    pub const fn new(expected: K, found: Token<K>) -> Self {
        Self { expected, found }
    }
}

impl<K: Copy> ParseError<K> {
    //! Properties

    /// Gets the expected token kind.
    pub const fn expected(self) -> K {
        self.expected
    }

    /// Gets the found token.
    pub const fn found(self) -> Token<K> {
        self.found
    }

    /// Gets the span of the found token.
    pub const fn span(self) -> Span {
        self.found.span()
    }
}

impl<K: Copy + TokenKind> Display for ParseError<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "expected {}, found {}",
            self.expected.label(),
            self.found.kind().label()
        )
    }
}

impl<K: Copy + TokenKind> std::error::Error for ParseError<K> {}
