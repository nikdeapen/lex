use crate::{match_prefix, match_prefix_optional, LexResult, Token};

// Whitespace is defined as US-ASCII spaces & tabs.

/// Parses optional non-empty whitespace.
pub fn whitespace_optional(token: Token) -> (Option<Token>, Token) {
    match_prefix_optional(token, |c| c == b' ' || c == b'\t')
}

/// Parses non-empty whitespace.
pub fn whitespace(token: Token) -> LexResult<Token, ()> {
    match_prefix(token, |c| c == b' ' || c == b'\t')
}
