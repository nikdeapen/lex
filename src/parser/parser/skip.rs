use super::Parser;

impl<K: Copy + PartialEq> Parser<K> {
    //! Skip

    /// Adds a token kind to skip during parsing.
    pub fn add_skip(&mut self, kind: K) {
        debug_assert!(self.tokens.last().unwrap().kind() != kind);

        self.skip.push(kind);
        self.skip_ignored();
    }

    /// Adds a token kind to skip during parsing. (builder pattern)
    #[must_use]
    pub fn with_skip(mut self, kind: K) -> Self {
        self.add_skip(kind);
        self
    }

    /// Advances past any skipped tokens.
    pub(in crate::parser::parser) fn skip_ignored(&mut self) {
        while self.skip.contains(&self.tokens[self.pos].kind()) {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::tests::{parser, K};
    use crate::parser::Parser;

    #[test]
    fn skip_advances_past_initial_whitespace() {
        let p: Parser<K> = parser("  x");
        assert_eq!(p.peek().kind(), K::Ident);
    }
}
