use crate::{LexResult, Token};

// A line-ending is defined as a CR, LF, or CRLF. The CRLF pair will always be joined.

/// Parses an optional line-ending.
pub fn line_ending_optional(token: Token) -> (Option<Token>, Token) {
    let b: &[u8] = token.bytes();
    let line_ending_len: usize = if b.is_empty() {
        0
    } else if b[0] == b'\r' {
        if b.len() != 1 && b[1] == b'\n' {
            2
        } else {
            1
        }
    } else if b[0] == b'\n' {
        1
    } else {
        0
    };
    if let (Some(line_ending), token) = token.split_optional(line_ending_len) {
        (Some(line_ending), token.with_new_line())
    } else {
        (None, token)
    }
}

/// Parses a line-ending.
pub fn line_ending(token: Token) -> LexResult<Token, ()> {
    if let (Some(line_ending), token) = line_ending_optional(token) {
        Ok((line_ending, token))
    } else {
        Err(token.into())
    }
}
