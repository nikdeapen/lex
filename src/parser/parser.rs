use crate::lexer::{Span, Token, TokenKind};
use crate::parser::{Checkpoint, ParseError};

/// A parser.
pub struct Parser<K> {
    source: String,
    tokens: Vec<Token<K>>,
    pos: usize,
    skip: Vec<K>,
    errors: Vec<ParseError>,
}

impl<K> Parser<K> {
    //! Construction

    /// Creates a new parser.
    pub fn new(source: String, tokens: Vec<Token<K>>) -> Self {
        Self {
            source,
            tokens,
            pos: 0,
            skip: Vec::default(),
            errors: Vec::default(),
        }
    }
}

impl<K: Copy + PartialEq> Parser<K> {
    //! Skip

    /// Adds a token kind to skip during parsing.
    pub fn add_skip(&mut self, kind: K) {
        self.skip.push(kind);
        self.skip_ignored();
    }

    /// Adds a token kind to skip during parsing. (builder pattern)
    pub fn with_skip(mut self, kind: K) -> Self {
        self.add_skip(kind);
        self
    }

    /// Advances past any skipped tokens.
    fn skip_ignored(&mut self) {
        while self.skip.contains(&self.tokens[self.pos].kind()) {
            self.pos += 1;
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
    pub fn peek(&self) -> &Token<K> {
        &self.tokens[self.pos]
    }

    /// Peeks at the token `n` positions ahead of the current position.
    pub fn lookahead(&self, n: usize) -> Option<&Token<K>> {
        self.tokens.get(self.pos + n)
    }

    /// Checks if the current token matches the `kind`.
    pub fn check(&self, kind: K) -> bool {
        self.peek().kind() == kind
    }
}

impl<K: Copy + PartialEq + TokenKind> Parser<K> {
    //! Consuming

    /// Advances the parser by one token and returns it.
    ///
    /// Returns `None` if at EOF.
    pub fn advance(&mut self) -> Option<Token<K>> {
        let pos: usize = self.pos;
        if self.tokens[pos].kind() == K::eof() {
            None
        } else {
            self.pos += 1;
            self.skip_ignored();
            Some(self.tokens[pos])
        }
    }

    /// Advances if the current token matches the `kind`.
    ///
    /// Returns `None` and records an auto-generated error if it does not match.
    pub fn expect(&mut self, kind: K) -> Option<Token<K>> {
        if self.check(kind) {
            self.advance()
        } else {
            let found: &'static str = self.peek().kind().label();
            let expected: &'static str = kind.label();
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
        while !self.check(kind) && !self.check(K::eof()) {
            self.advance();
        }
    }
}

impl<K: Copy + PartialEq> Parser<K> {
    //! Checkpoints

    /// Creates a checkpoint at the current position.
    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint::new(self.pos, self.errors.len())
    }

    /// Restores the parser to a previous checkpoint, rewinding position and discarding errors.
    pub fn restore(&mut self, checkpoint: Checkpoint) {
        self.pos = checkpoint.pos();
        self.errors.truncate(checkpoint.error_count());
        self.skip_ignored();
    }
}

impl<K: Copy> Parser<K> {
    //! Errors

    /// Records an error at the current token's span.
    pub fn error(&mut self, message: impl Into<String>) {
        let span: Span = self.tokens[self.pos].span();
        self.errors.push(ParseError::new(span, message));
    }

    /// Gets the collected errors.
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    /// Consumes the parser and returns the collected errors.
    pub fn into_errors(self) -> Vec<ParseError> {
        self.errors
    }
}

impl<K> Parser<K> {
    //! Source

    /// Gets the source text.
    pub fn source(&self) -> &str {
        &self.source
    }
}
