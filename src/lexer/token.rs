use crate::lexer::Span;
use std::fmt::{Display, Formatter};

/// A lexical token.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Token<K> {
    kind: K,
    span: Span,
}

impl<K> Token<K> {
    //! Construction

    /// Creates a new lexical token.
    pub const fn new(kind: K, span: Span) -> Self {
        Self { kind, span }
    }
}

impl<K: Copy> Token<K> {
    //! Properties

    /// Gets the token kind.
    pub const fn kind(self) -> K {
        self.kind
    }

    /// Gets the token span.
    pub const fn span(self) -> Span {
        self.span
    }
}

impl<K: Display> Display for Token<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "token[{}, {}]", self.kind, self.span)
    }
}
