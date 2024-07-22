use crate::{rest_of_line, LexResult, Token};

/// Parses an optional line-comments.
/// - Returns the comments without the delimiter
pub fn line_comment_optional<'a>(
    token: Token<'a>,
    delimiter: &str,
) -> (Option<Token<'a>>, Token<'a>) {
    if token.value().starts_with(delimiter) {
        let (_delimiter, token) = token.split(delimiter.len());
        let (comment, _line_ending, token) = rest_of_line(token);
        (Some(comment), token)
    } else {
        (None, token)
    }
}

/// Parses an optional line-comments.
/// - Returns the comments without the delimiter
pub fn line_comment<'a>(token: Token<'a>, delimiter: &str) -> LexResult<'a, Token<'a>, ()> {
    if let (Some(comment), token) = line_comment_optional(token, delimiter) {
        Ok((comment, token))
    } else {
        Err(token.into())
    }
}
