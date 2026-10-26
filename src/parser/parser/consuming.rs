use super::Parser;
use crate::lexer::{Token, TokenKind};

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
    /// Returns `None` without recording an error if it does not match.
    pub fn accept(&mut self, kind: K) -> Option<Token<K>> {
        if self.check(kind) {
            self.advance()
        } else {
            None
        }
    }

    /// Advances if the current token matches the `kind`.
    ///
    /// Returns `None` and records an auto-generated error if it does not match.
    pub fn expect(&mut self, kind: K) -> Option<Token<K>> {
        if self.check(kind) {
            self.advance()
        } else {
            let found: String = self.peek().kind().label();
            let expected: String = kind.label();
            let message: String = format!("expected {expected}, found {found}");
            self.error(message);
            None
        }
    }

    /// Advances if the current token matches the `kind`.
    ///
    /// Returns `None` and records a custom error if it does not match.
    pub fn expect_with(&mut self, kind: K, message: impl Into<String>) -> Option<Token<K>> {
        if self.check(kind) {
            self.advance()
        } else {
            self.error(message);
            None
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
    use crate::lexer::Token;
    use crate::parser::Parser;
    use crate::parser::parser::tests::{K, parser};

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
        assert!(!p.has_errors());
        assert_eq!(p.peek().kind(), K::Ident);
    }

    #[test]
    fn expect_consumes_matching_kind() {
        let mut p: Parser<K> = parser("x;");
        assert!(p.expect(K::Ident).is_some());
        assert!(!p.has_errors());
    }

    #[test]
    fn expect_records_error_on_mismatch() {
        let mut p: Parser<K> = parser("x;");
        assert!(p.expect(K::Eq).is_none());
        assert!(p.has_errors());
        assert_eq!(p.errors()[0].message(), "expected Eq, found Ident");
    }

    #[test]
    fn expect_with_records_custom_message() {
        let mut p: Parser<K> = parser("x;");
        assert!(p.expect_with(K::Eq, "expected '='").is_none());
        assert_eq!(p.errors()[0].message(), "expected '='");
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
