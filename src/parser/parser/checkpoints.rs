use super::Parser;
use crate::parser::checkpoint::Checkpoint;

impl<K: Copy + PartialEq> Parser<K> {
    //! Checkpoints

    /// Creates a checkpoint at the current position.
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint::new(self.pos, self.errors.len())
    }

    /// Restores the parser to a previous checkpoint, rewinding position and discarding errors.
    pub fn restore(&mut self, checkpoint: Checkpoint) {
        debug_assert!(checkpoint.pos() < self.tokens.len());
        self.pos = checkpoint.pos();
        self.errors.truncate(checkpoint.error_count());
        self.skip_ignored();
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::parser::checkpoint::Checkpoint;
    use crate::parser::parser::tests::{K, parser};

    #[test]
    fn checkpoint_and_restore() {
        let mut p: Parser<K> = parser("x = y;");
        let cp: Checkpoint = p.checkpoint();
        p.advance();
        p.advance();
        assert_eq!(p.peek().kind(), K::Ident);
        p.restore(cp);
        assert_eq!(p.peek().kind(), K::Ident);
        assert_eq!(p.peek().span().text(p.source()), "x");
    }

    #[test]
    fn restore_discards_errors() {
        let mut p: Parser<K> = parser("x;");
        let cp: Checkpoint = p.checkpoint();
        p.expect(K::Eq);
        assert!(p.has_errors());
        p.restore(cp);
        assert!(!p.has_errors());
    }
}
