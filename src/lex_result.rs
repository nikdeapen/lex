use crate::{LexError, Token};

/// The result of parsing a value.
pub type LexResult<'a, T, E> = Result<(T, Token<'a>), LexError<'a, E>>;
