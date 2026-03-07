use crate::lexer::{Rule, Span, Token, TokenKind};

/// A lexer. Converts source text into a sequence of tokens using ordered rules.
#[derive(Clone)]
pub struct Lexer<K> {
    rules: Vec<Rule<K>>,
}

impl<K> Default for Lexer<K> {
    fn default() -> Self {
        Self {
            rules: Vec::default(),
        }
    }
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

        let mut tokens: Vec<Token<K>> = Vec::default();
        let mut pos: usize = 0;

        while pos < source.len() {
            let remaining: &str = &source[pos..];
            let (kind, len): (K, usize) = self.match_rule(remaining);
            debug_assert!(len > 0);
            debug_assert!(source.is_char_boundary(pos + len));
            let span: Span = Span::new(pos as u32, len as u32);
            tokens.push(Token::new(kind, span));
            pos += len;
        }

        let eof_span: Span = Span::new(pos as u32, 0);
        tokens.push(Token::new(K::eof(), eof_span));
        tokens
    }

    /// Matches the first rule against the `remaining` source.
    fn match_rule(&self, remaining: &str) -> (K, usize) {
        for rule in &self.rules {
            if let Some(len) = rule.try_match(remaining) {
                if len > 0 {
                    return (rule.kind(), len);
                }
            }
        }
        let c: char = remaining.chars().next().unwrap();
        (K::unknown(), c.len_utf8())
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::matchers::{digits, ident, whitespace};
    use crate::lexer::{Lexer, Span, Token, TokenKind};
    use crate::literal;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    enum Kind {
        Whitespace,
        Ident,
        Int,
        LBrace,
        RBrace,
        Semi,
        Eq,
        Unknown,
        Eof,
    }

    impl TokenKind for Kind {
        fn unknown() -> Self {
            Kind::Unknown
        }

        fn eof() -> Self {
            Kind::Eof
        }

        fn label(&self) -> &'static str {
            match self {
                Kind::Whitespace => "whitespace",
                Kind::Ident => "identifier",
                Kind::Int => "integer",
                Kind::LBrace => "'{'",
                Kind::RBrace => "'}'",
                Kind::Semi => "';'",
                Kind::Eq => "'='",
                Kind::Unknown => "unknown",
                Kind::Eof => "end of file",
            }
        }
    }

    fn test_lexer() -> Lexer<Kind> {
        Lexer::default()
            .with_rule(Kind::Whitespace, whitespace)
            .with_rule(Kind::Ident, ident)
            .with_rule(Kind::Int, digits)
            .with_rule(Kind::LBrace, literal!("{"))
            .with_rule(Kind::RBrace, literal!("}"))
            .with_rule(Kind::Semi, literal!(";"))
            .with_rule(Kind::Eq, literal!("="))
    }

    #[test]
    fn fn_lex() {
        let lexer: Lexer<Kind> = test_lexer();
        let source: &str = "let x = 42;";
        let tokens: Vec<Token<Kind>> = lexer.lex(source);

        let expected: &[(Kind, &str)] = &[
            (Kind::Ident, "let"),
            (Kind::Whitespace, " "),
            (Kind::Ident, "x"),
            (Kind::Whitespace, " "),
            (Kind::Eq, "="),
            (Kind::Whitespace, " "),
            (Kind::Int, "42"),
            (Kind::Semi, ";"),
            (Kind::Eof, ""),
        ];

        assert_eq!(tokens.len(), expected.len());
        for (token, (kind, text)) in tokens.iter().zip(expected) {
            assert_eq!(token.kind(), *kind);
            assert_eq!(token.text(source), *text);
        }
    }

    #[test]
    fn fn_lex_unknown() {
        let lexer: Lexer<Kind> = test_lexer();
        let source: &str = "x @ y";
        let tokens: Vec<Token<Kind>> = lexer.lex(source);

        let expected: &[(Kind, &str)] = &[
            (Kind::Ident, "x"),
            (Kind::Whitespace, " "),
            (Kind::Unknown, "@"),
            (Kind::Whitespace, " "),
            (Kind::Ident, "y"),
            (Kind::Eof, ""),
        ];

        assert_eq!(tokens.len(), expected.len());
        for (token, (kind, text)) in tokens.iter().zip(expected) {
            assert_eq!(token.kind(), *kind);
            assert_eq!(token.text(source), *text);
        }
    }

    #[test]
    fn fn_lex_empty() {
        let lexer: Lexer<Kind> = test_lexer();
        let tokens: Vec<Token<Kind>> = lexer.lex("");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind(), Kind::Eof);
    }

    #[test]
    fn fn_lex_spans() {
        let lexer: Lexer<Kind> = test_lexer();
        let source: &str = "a 1";
        let tokens: Vec<Token<Kind>> = lexer.lex(source);

        assert_eq!(tokens[0].span(), Span::new(0, 1));
        assert_eq!(tokens[1].span(), Span::new(1, 1));
        assert_eq!(tokens[2].span(), Span::new(2, 1));
    }
}
