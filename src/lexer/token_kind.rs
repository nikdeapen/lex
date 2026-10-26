use std::fmt::Debug;

/// A kind of lexical token.
pub trait TokenKind: Debug {
    /// Gets the kind of lexical token for unrecognized tokens.
    fn unrecognized() -> Self;

    /// Gets the end-of-file token kind.
    fn end_of_file() -> Self;

    /// Gets the display label for this token kind. (ex: `"Identifier"`, `"EndOfFile"`)
    fn label(&self) -> String {
        format!("{:?}", self)
    }
}
