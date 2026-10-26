use super::Parser;
use crate::lexer::{Span, Token};
use crate::parser::comment_config::CommentConfig;

impl<K: Copy + PartialEq> Parser<K> {
    //! Comments

    /// Configures line comment extraction.
    ///
    /// The `kind` and `delimiter` should match the [line_comment!] matcher used in the lexer.
    #[must_use]
    pub fn with_line_comment(mut self, kind: K, delimiter: &str) -> Self {
        debug_assert!(!delimiter.is_empty());

        self.comment = Some(CommentConfig {
            kind,
            delimiter_len: delimiter.len(),
        });
        self
    }

    /// Gets the leading comment spans before the current position.
    ///
    /// Each span covers the text after the delimiter and before the line ending. Only the line
    /// comments directly above the current token are included: each comment starts its own line
    /// and no blank line separates it from the line below.
    pub fn leading_comments(&self) -> Vec<Span> {
        let config: CommentConfig<K> = match self.comment {
            Some(c) => c,
            None => return Vec::default(),
        };

        let mut comments: Vec<Span> = Vec::default();
        let mut below: u32 = self.tokens[self.pos].span().offset();
        let mut i: usize = self.pos;

        while i > 0 {
            i -= 1;
            let token: Token<K> = self.tokens[i];
            if self.skip.contains(&token.kind()) && token.kind() != config.kind {
                continue;
            }
            if token.kind() != config.kind || !self.is_directly_above(token.span(), below) {
                break;
            }

            let offset: u32 = token.span().offset() + config.delimiter_len as u32;
            let mut len: u32 = token.span().len() - config.delimiter_len as u32;

            // trim trailing whitespace
            let bytes: &[u8] = self.source.as_bytes();
            while len > 0 && bytes[(offset + len - 1) as usize].is_ascii_whitespace() {
                len -= 1;
            }

            comments.push(Span::new(offset, len));
            below = token.span().offset();
        }

        comments.reverse();
        comments
    }

    /// Checks if the `comment` starts its own line & is on the line directly above the `below`
    /// offset.
    fn is_directly_above(&self, comment: Span, below: u32) -> bool {
        let start: usize = comment.offset() as usize;
        let line_start: usize = self.source[..start].rfind('\n').map_or(0, |i| i + 1);
        let starts_line: bool = self.source[line_start..start].trim().is_empty();
        let newlines: usize = self.source[start..below as usize].matches('\n').count();
        starts_line && newlines == 1
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::matchers::{ident, whitespace};
    use crate::lexer::{Span, Token};
    use crate::parser::Parser;
    use crate::{line_comment, literal};

    crate::lexer! {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        enum CK {
            Whitespace: whitespace,
            LineComment: line_comment!("//"),
            Ident: ident,
            Eq: literal!("="),
            Semi: literal!(";"),
        }
    }

    fn comment_parser(source: &str) -> Parser<CK> {
        let source: String = source.to_string();
        let tokens: Vec<Token<CK>> = CK::lexer().lex(&source);
        Parser::new(source, tokens)
            .with_skip(CK::Whitespace)
            .with_skip(CK::LineComment)
            .with_line_comment(CK::LineComment, "//")
    }

    #[test]
    fn leading_comments_without_config() {
        let source: String = "// comment\nx".to_string();
        let tokens: Vec<Token<CK>> = CK::lexer().lex(&source);
        let p: Parser<CK> = Parser::new(source, tokens).with_skip(CK::Whitespace);
        assert!(p.leading_comments().is_empty());
    }

    #[test]
    fn leading_comments_single() {
        let p: Parser<CK> = comment_parser("// hello\nx");
        let comments: Vec<Span> = p.leading_comments();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text(p.source()), " hello");
    }

    #[test]
    fn leading_comments_multiple() {
        let p: Parser<CK> = comment_parser("// first\n// second\nx");
        let comments: Vec<Span> = p.leading_comments();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].text(p.source()), " first");
        assert_eq!(comments[1].text(p.source()), " second");
    }

    #[test]
    fn leading_comments_trims_trailing_whitespace() {
        let p: Parser<CK> = comment_parser("// hello  \nx");
        let comments: Vec<Span> = p.leading_comments();
        assert_eq!(comments[0].text(p.source()), " hello");
    }

    #[test]
    fn leading_comments_indented() {
        let p: Parser<CK> = comment_parser("    // first\n    // second\n    x");
        let comments: Vec<Span> = p.leading_comments();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].text(p.source()), " first");
        assert_eq!(comments[1].text(p.source()), " second");
    }

    #[test]
    fn leading_comments_crlf() {
        let p: Parser<CK> = comment_parser("// first\r\n// second\r\nx");
        let comments: Vec<Span> = p.leading_comments();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].text(p.source()), " first");
        assert_eq!(comments[1].text(p.source()), " second");
    }

    #[test]
    fn leading_comments_skips_trailing_comment() {
        let mut p: Parser<CK> = comment_parser("x; // trailing\ny");
        p.advance();
        p.advance();
        assert!(p.leading_comments().is_empty());
    }

    #[test]
    fn leading_comments_stops_at_blank_line() {
        let p: Parser<CK> = comment_parser("// far\n\nx");
        assert!(p.leading_comments().is_empty());
    }

    #[test]
    fn leading_comments_stops_at_blank_line_between_comments() {
        let p: Parser<CK> = comment_parser("// first\n\n// second\nx");
        let comments: Vec<Span> = p.leading_comments();
        assert_eq!(comments.len(), 1);
        assert_eq!(comments[0].text(p.source()), " second");
    }
}
