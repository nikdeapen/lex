use crate::lexer::{Span, Token, TokenKind};
use crate::parser::CommentConfig;

mod checkpoints;
mod comments;
mod consuming;
mod skip;

/// A parser.
pub struct Parser<K> {
    pub(in crate::parser::parser) source: String,
    pub(in crate::parser::parser) tokens: Vec<Token<K>>,
    pub(in crate::parser::parser) pos: usize,
    pub(in crate::parser::parser) skip: Vec<K>,
    pub(in crate::parser::parser) comment: Option<CommentConfig<K>>,
}

impl<K: Copy + PartialEq + TokenKind> Parser<K> {
    //! Construction

    /// Creates a new parser.
    pub fn new(source: String, tokens: Vec<Token<K>>) -> Self {
        debug_assert!(!tokens.is_empty() && tokens.last().unwrap().kind() == K::end_of_file());

        Self {
            source,
            tokens,
            pos: 0,
            skip: Vec::default(),
            comment: None,
        }
    }
}

impl<K: Copy + PartialEq> Parser<K> {
    //! Position

    /// Gets the current position in the token stream.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Peeks at the current token.
    pub fn peek(&self) -> Token<K> {
        self.tokens[self.pos]
    }

    /// Peeks at the `n`th non-skipped token from the current position. (0-indexed)
    ///
    /// `lookahead(0)` is equivalent to `peek()`.
    pub fn lookahead(&self, n: usize) -> Option<Token<K>> {
        let mut remaining: usize = n;
        let mut i: usize = self.pos;
        while i < self.tokens.len() {
            if !self.skip.contains(&self.tokens[i].kind()) {
                if remaining == 0 {
                    return Some(self.tokens[i]);
                }
                remaining -= 1;
            }
            i += 1;
        }
        None
    }

    /// Checks if the current token matches the `kind`.
    pub fn check(&self, kind: K) -> bool {
        self.peek().kind() == kind
    }
}

impl<K> Parser<K> {
    //! Source

    /// Gets the source text.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Gets the text for a `span`.
    pub fn text(&self, span: Span) -> &str {
        span.text(&self.source)
    }
}

impl<K> Parser<K> {
    //! Tokens

    /// Gets the token stream.
    pub fn tokens(&self) -> &[Token<K>] {
        &self.tokens
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::matchers::{ident, whitespace};
    use crate::lexer::{Span, Token};
    use crate::literal;
    use crate::parser::Parser;

    crate::lexer! {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub(in crate::parser::parser) enum K {
            Whitespace: whitespace,
            Ident: ident,
            Eq: literal!("="),
            Semi: literal!(";"),
        }
    }

    pub(in crate::parser::parser) fn parser(source: &str) -> Parser<K> {
        let source: String = source.to_string();
        let tokens: Vec<Token<K>> = K::lexer().lex(&source);
        Parser::new(source, tokens).with_skip(K::Whitespace)
    }

    // Position

    #[test]
    fn peek_returns_current_token() {
        let p: Parser<K> = parser("x = y");
        assert_eq!(p.peek().kind(), K::Ident);
    }

    #[test]
    fn check_matches_current_kind() {
        let p: Parser<K> = parser("x");
        assert!(p.check(K::Ident));
        assert!(!p.check(K::Eq));
    }

    #[test]
    fn lookahead_zero_equals_peek() {
        let p: Parser<K> = parser("x = y");
        assert_eq!(p.lookahead(0).unwrap().kind(), p.peek().kind());
    }

    #[test]
    fn lookahead_skips_whitespace() {
        let p: Parser<K> = parser("x = y");
        assert_eq!(p.lookahead(1).unwrap().kind(), K::Eq);
        assert_eq!(p.lookahead(2).unwrap().kind(), K::Ident);
    }

    #[test]
    fn lookahead_past_end_returns_none() {
        let p: Parser<K> = parser("x");
        assert!(p.lookahead(10).is_none());
    }

    // Source & Tokens

    #[test]
    fn source_returns_full_source() {
        let p: Parser<K> = parser("x = y");
        assert_eq!(p.source(), "x = y");
    }

    #[test]
    fn text_returns_span_text() {
        let p: Parser<K> = parser("x = y");
        let span: Span = p.peek().span();
        assert_eq!(p.text(span), "x");
    }

    #[test]
    fn tokens_returns_full_stream() {
        let p: Parser<K> = parser("x;");
        let tokens: &[Token<K>] = p.tokens();
        assert_eq!(tokens.last().unwrap().kind(), K::EndOfFile);
    }
}
