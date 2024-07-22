use crate::{line_ending_optional, whitespace_optional, LexResult, Token};

/// Parses optional non-empty whitespace & line-endings.
pub fn white_lines_optional(token: Token) -> (Option<Token>, Token) {
    let mut rem: Token = token;
    loop {
        let mut matched: bool = false;

        if let (Some(_whitespace), r) = whitespace_optional(rem) {
            matched = true;
            rem = r;
        }

        if let (Some(_line_ending), r) = line_ending_optional(rem) {
            matched = true;
            rem = r;
        }

        if !matched {
            return if token.len() == rem.len() {
                (None, token)
            } else {
                let (a, _b) = token.split(token.len() - rem.len());
                (Some(a), rem)
            };
        }
    }
}

/// Parses non-empty whitespace & line-endings.
pub fn white_lines(token: Token) -> LexResult<Token, ()> {
    if let (Some(white_lines), token) = white_lines_optional(token) {
        Ok((white_lines, token))
    } else {
        Err(token.into())
    }
}
