use crate::lexer::{Rule, Span, Token, TokenKind};

/// A lexer. Converts source text into a sequence of tokens using ordered rules.
#[derive(Clone, Default)]
pub struct Lexer<K> {
    rules: Vec<Rule<K>>,
}

impl<K> Lexer<K> {
    //! Rules

    /// Adds a rule to the lexer.
    pub fn add_rule(&mut self, kind: K, matcher: fn(&str) -> Option<usize>) {
        self.rules.push(Rule::new(kind, matcher));
    }

    /// Adds a rule to the lexer. (builder pattern)
    pub fn with_rule(mut self, kind: K, matcher: fn(&str) -> Option<usize>) -> Self {
        self.add_rule(kind, matcher);
        self
    }
}

impl<K: Copy + TokenKind> Lexer<K> {
    //! Lexing

    /// Lexes the `source` into a sequence of tokens.
    pub fn lex(&self, source: &str) -> Vec<Token<K>> {
        debug_assert!(source.len() <= u32::MAX as usize);

        let mut tokens: Vec<Token<K>> = Vec::new();
        let mut pos: usize = 0;

        while pos < source.len() {
            let remaining: &str = &source[pos..];
            let (kind, len): (K, usize) = self.match_rule(remaining);
            debug_assert!(source.is_char_boundary(pos + len));
            let span: Span = Span::new(pos as u32, len as u32);
            tokens.push(Token::new(kind, span));
            pos += len;
        }

        tokens
    }

    /// Matches the first rule against the `remaining` source.
    fn match_rule(&self, remaining: &str) -> (K, usize) {
        for rule in &self.rules {
            if let Some(len) = rule.try_match(remaining) {
                return (rule.kind(), len);
            }
        }
        let c: char = remaining.chars().next().unwrap();
        (K::unknown(), c.len_utf8())
    }
}
