use crate::{line_ending_optional, Token};

/// Parses the rest of the line.
///
/// Returns `(line, optional_line_ending, rest_of_token)`.
pub fn rest_of_line(token: Token) -> (Token, Option<Token>, Token) {
    let line_len: usize = token
        .bytes()
        .iter()
        .position(|c| *c == b'\r' || *c == b'\n')
        .unwrap_or(token.len());
    let (line, token) = token.split(line_len);
    let (line_ending, token) = line_ending_optional(token);
    (line, line_ending, token)
}
