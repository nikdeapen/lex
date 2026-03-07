use crate::lexer::{Span, Token, TokenKind};
use crate::parser::{Checkpoint, ParseError};

/// A line comment configuration.
#[derive(Copy, Clone)]
struct CommentConfig<K> {
    kind: K,
    delimiter_len: usize,
}

/// A parser.
pub struct Parser<K> {
    source: String,
    tokens: Vec<Token<K>>,
    pos: usize,
    skip: Vec<K>,
    comment: Option<CommentConfig<K>>,
    errors: Vec<ParseError>,
}

impl<K: Copy + PartialEq + TokenKind> Parser<K> {
    //! Construction

    /// Creates a new parser.
    pub fn new(source: String, tokens: Vec<Token<K>>) -> Self {
        debug_assert!(!tokens.is_empty() && tokens.last().unwrap().kind() == K::eof());

        Self {
            source,
            tokens,
            pos: 0,
            skip: Vec::default(),
            comment: None,
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
    //! Comments

    /// Configures line comment extraction.
    pub fn with_line_comment(mut self, kind: K, delimiter: &str) -> Self {
        self.comment = Some(CommentConfig {
            kind,
            delimiter_len: delimiter.len(),
        });
        self
    }

    /// Gets the leading comment spans before the current position.
    ///
    /// Each span covers the text after the delimiter and before the line ending. Walks backward
    /// through the token stream, skipping whitespace, collecting consecutive comment tokens.
    pub fn leading_comments(&self) -> Vec<Span> {
        let config: CommentConfig<K> = match self.comment {
            Some(c) => c,
            None => return Vec::default(),
        };

        let mut comments: Vec<Span> = Vec::default();
        let mut i: usize = self.pos;

        while i > 0 {
            i -= 1;
            let token: Token<K> = self.tokens[i];
            if self.skip.contains(&token.kind()) && token.kind() != config.kind {
                continue;
            }
            if token.kind() != config.kind {
                break;
            }

            let offset: u32 = token.span().offset() + config.delimiter_len as u32;
            let mut len: u32 = token.span().len() - config.delimiter_len as u32;

            // trim trailing newline
            let text: &str = Span::new(offset, len).text(&self.source);
            if text.ends_with('\n') {
                len -= 1;
            }
            if Span::new(offset, len).text(&self.source).ends_with('\r') {
                len -= 1;
            }

            comments.push(Span::new(offset, len));
        }

        comments.reverse();
        comments
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

    /// Peeks at the token `n` non-skipped positions ahead of the current position.
    pub fn lookahead(&self, n: usize) -> Option<Token<K>> {
        let mut remaining: usize = n;
        let mut i: usize = self.pos + 1;
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

    /// Gets the text for a `span`.
    pub fn text(&self, span: Span) -> &str {
        span.text(&self.source)
    }
}
