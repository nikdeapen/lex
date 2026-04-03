/// A lexer rule. Maps a matcher function to a kind of lexical token.
#[derive(Copy, Clone, Debug)]
pub struct Rule<K> {
    kind: K,
    matcher: fn(&str) -> Option<usize>,
}

impl<K> Rule<K> {
    //! Construction

    /// Creates a new rule.
    pub const fn new(kind: K, matcher: fn(&str) -> Option<usize>) -> Self {
        Self { kind, matcher }
    }
}

impl<K: Copy> Rule<K> {
    //! Properties

    /// Gets the token kind.
    pub(in crate::lexer) fn kind(self) -> K {
        self.kind
    }
}

impl<K> Rule<K> {
    //! Matching

    /// Attempts to match the `source`. Returns the number of bytes consumed.
    pub(in crate::lexer) fn try_match(self, source: &str) -> Option<usize> {
        (self.matcher)(source)
    }
}
