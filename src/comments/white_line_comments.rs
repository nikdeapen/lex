use crate::{white_lines_optional, LexResult, Token, line_comment_optional};

/// Parses optional non-empty whitespace, line-endings, & line-comments.
pub fn white_line_comments_optional<'a>(
    token: Token<'a>,
    delimiter: &str,
) -> (Option<Token<'a>>, Token<'a>) {
    let mut rem: Token = token;
    loop {
        let mut matched: bool = false;

        if let (Some(_white_lines), r) = white_lines_optional(rem) {
            matched = true;
            rem = r;
        }

        if let (Some(_comment), r) = line_comment_optional(rem, delimiter) {
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

/// Parses non-empty whitespace, line-endings, & line-comments.
pub fn white_line_comments<'a>(token: Token<'a>, delimiter: &str) -> LexResult<'a, Token<'a>, ()> {
    if let (Some(white_line_comments), token) = white_line_comments_optional(token, delimiter) {
        Ok((white_line_comments, token))
    } else {
        Err(token.into())
    }
}
