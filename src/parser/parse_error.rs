use crate::lexer::Span;

/// A parse error.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct ParseError {
    span: Span,
    message: String,
}

impl ParseError {
    //! Construction

    /// Creates a new parse error.
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl ParseError {
    //! Properties

    /// Gets the error span.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Gets the error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}
