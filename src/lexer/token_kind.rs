/// A kind of lexical token.
pub trait TokenKind {
    /// Gets the kind of token to be used when there are no matching rules.
    fn unknown() -> Self;

    /// Gets the end-of-file token kind.
    fn eof() -> Self;

    /// Gets the display label for this token kind. (e.g. `"identifier"`, `"'{'"`, `"end of file"`)
    fn label(&self) -> &'static str;
}
