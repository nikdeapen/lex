use super::Parser;
use crate::lexer::{Token, TokenKind};
use crate::parser::ParseError;

impl<K: Copy + PartialEq + TokenKind> Parser<K> {
    //! Consuming

    /// Advances the parser by one token and returns it.
    ///
    /// Returns `None` if at EOF.
    pub fn advance(&mut self) -> Option<Token<K>> {
        let pos: usize = self.pos;
        if self.tokens[pos].kind() == K::end_of_file() {
            None
        } else {
            self.pos += 1;
            self.skip_ignored();
            Some(self.tokens[pos])
        }
    }

    /// Advances if the current token matches the `kind`.
    ///
    /// Returns `None` if it does not match.
    pub fn accept(&mut self, kind: K) -> Option<Token<K>> {
        self.expect(kind).ok()
    }

    /// Advances if the current token matches the `kind`.
    ///
    /// Returns an error with the current token if it does not match.
    pub fn expect(&mut self, kind: K) -> Result<Token<K>, ParseError<K>> {
        let token: Token<K> = self.peek();
        if token.kind() == kind {
            self.advance();
            Ok(token)
        } else {
            Err(ParseError::new(kind, token))
        }
    }

    /// Advances until the current token matches the `kind` or EOF is reached.
    pub fn skip_until(&mut self, kind: K) {
        while !self.check(kind) && !self.check(K::end_of_file()) {
            self.advance();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Span, Token};
    use crate::parser::parser::tests::{K, parser};
    use crate::parser::{ParseError, Parser};

    #[test]
    fn advance_returns_token_and_moves_forward() {
        let mut p: Parser<K> = parser("x = y");
        let token: Token<K> = p.advance().unwrap();
        assert_eq!(token.kind(), K::Ident);
        assert_eq!(token.span().text(p.source()), "x");
        assert_eq!(p.peek().kind(), K::Eq);
    }

    #[test]
    fn advance_at_eof_returns_none() {
        let mut p: Parser<K> = parser("");
        assert!(p.advance().is_none());
    }

    #[test]
    fn accept_consumes_matching_kind() {
        let mut p: Parser<K> = parser("x;");
        assert!(p.accept(K::Ident).is_some());
        assert_eq!(p.peek().kind(), K::Semi);
    }

    #[test]
    fn accept_does_not_consume_on_mismatch() {
        let mut p: Parser<K> = parser("x;");
        assert!(p.accept(K::Eq).is_none());
        assert_eq!(p.peek().kind(), K::Ident);
    }

    #[test]
    fn accept_matches_eof() {
        let mut p: Parser<K> = parser("");
        assert_eq!(p.accept(K::EndOfFile).unwrap().kind(), K::EndOfFile);
    }

    #[test]
    fn expect_consumes_matching_kind() {
        let mut p: Parser<K> = parser("x;");
        assert_eq!(p.expect(K::Ident).unwrap().kind(), K::Ident);
        assert_eq!(p.peek().kind(), K::Semi);
    }

    #[test]
    fn expect_returns_error_on_mismatch() {
        let mut p: Parser<K> = parser("x;");
        let error: ParseError<K> = p.expect(K::Eq).unwrap_err();
        assert_eq!(error.expected(), K::Eq);
        assert_eq!(error.found().kind(), K::Ident);
        assert_eq!(error.span(), Span::new(0, 1));
        assert_eq!(error.to_string(), "expected Eq, found Ident");
        assert_eq!(p.peek().kind(), K::Ident);
    }

    #[test]
    fn expect_matches_eof() {
        let mut p: Parser<K> = parser("x");
        p.advance();
        assert_eq!(p.expect(K::EndOfFile).unwrap().kind(), K::EndOfFile);
    }

    #[test]
    fn skip_until_stops_at_target() {
        let mut p: Parser<K> = parser("x = y;");
        p.skip_until(K::Semi);
        assert_eq!(p.peek().kind(), K::Semi);
    }

    #[test]
    fn skip_until_stops_at_eof() {
        let mut p: Parser<K> = parser("x = y");
        p.skip_until(K::Semi);
        assert_eq!(p.peek().kind(), K::EndOfFile);
    }
}
