use crate::{LexResult, Token};

/// Parses an optional exact value.
pub fn exact_optional<'a>(token: Token<'a>, value: &str) -> (Option<Token<'a>>, Token<'a>) {
    if token.value().starts_with(value) {
        token.split_optional(value.len())
    } else {
        (None, token)
    }
}

/// Parses an exact value.
pub fn exact<'a>(token: Token<'a>, value: &str) -> LexResult<'a, Token<'a>, ()> {
    if token.value().starts_with(value) {
        Ok(token.split(value.len()))
    } else {
        Err(token.into())
    }
}
