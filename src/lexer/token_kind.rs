/// A kind of lexical token.
pub trait TokenKind {
    /// Returns the kind of token to be used when there are no matching rules.
    fn unknown() -> Self;
}
