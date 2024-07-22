use crate::{match_prefix, match_prefix_optional, LexResult, Token};

// A symbol is defined as a non-empty sequence of US-ASCII letters, numbers, & underscores.

/// Parses an optional symbol.
pub fn symbol_optional(token: Token) -> (Option<Token>, Token) {
    match_prefix_optional(token, |c| c.is_ascii_alphanumeric() || c == b'_')
}

/// Parses a symbol.
pub fn symbol(token: Token) -> LexResult<Token, ()> {
    match_prefix(token, |c| c.is_ascii_alphanumeric() || c == b'_')
}
