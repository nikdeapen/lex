use crate::{LexResult, Token};

/// Matches an optional non-empty prefix of the token.
pub fn match_prefix_optional<F>(token: Token, filter: F) -> (Option<Token>, Token)
where
    F: Fn(u8) -> bool,
{
    let prefix_len: usize = token
        .bytes()
        .iter()
        .position(|c| !filter(*c))
        .unwrap_or(token.len());
    token.split_optional(prefix_len)
}

/// Matches a non-empty prefix of the token.
pub fn match_prefix<F>(token: Token, filter: F) -> LexResult<Token, ()>
where
    F: Fn(u8) -> bool,
{
    if let (Some(prefix), token) = match_prefix_optional(token, filter) {
        Ok((prefix, token))
    } else {
        Err(token.into())
    }
}
