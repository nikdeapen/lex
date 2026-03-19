use crate::lexer::Span;

/// A lexical token.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Token<K> {
    kind: K,
    span: Span,
}

impl<K> Token<K> {
    //! Construction

    /// Creates a new token.
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

    /// Gets the token text from the `source`.
    pub fn text<'src>(self, source: &'src str) -> &'src str {
        self.span.text(source)
    }
}
