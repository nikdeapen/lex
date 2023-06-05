use crate::Kind::{Controls, LineEnding, NonAscii, Special, Symbol, Whitespace};
use crate::{Kind, Lex, Token};

/// Responsible for tokenizing source code.
#[derive(Copy, Clone, Debug)]
pub struct Lexer {
    token_buffer_size: usize,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            token_buffer_size: 4 * 1024,
        }
    }
}

impl Lexer {
    //! Lexing

    /// Tokenizes the source code. Results the lex or an error if the source code was too large.
    /// Source code is too large if the number of lines or number of bytes within a single line
    /// exceeds u32::MAX (4 GiB or ~4 billion).
    pub fn lex<'a>(&self, source: &'a str) -> Result<Lex<'a>, ()> {
        let mut remaining: &'a str = source;
        let mut line: u32 = 0;
        let mut position: u32 = 0;
        let mut tokens: Vec<Token<'a>> = Vec::with_capacity(self.token_buffer_size);
        while !remaining.is_empty() {
            let (kind, len) = Self::next_kind_and_len(remaining);
            if len > u32::MAX as usize {
                return Err(());
            }
            let token: Token<'a> = Token::new(kind, &remaining[..len], line, position);
            tokens.push(token);
            if token.kind() == LineEnding {
                position = 0;
                line = line.checked_add(1).ok_or(())?;
            } else {
                position = position.checked_add(len as u32).ok_or(())?;
            }
            remaining = &remaining[len..];
        }
        Ok(Lex::new(source, tokens))
    }
}

impl Lexer {
    //! Lexing Utilities

    /// Gets the next token kind and length from the string.
    pub fn next_kind_and_len(s: &str) -> (Kind, usize) {
        debug_assert!(!s.is_empty());

        let b: &[u8] = s.as_bytes();
        let first: u8 = b[0];
        match first {
            c if c > 127 => (NonAscii, Self::prefix_len(b, |c| c > 127)),
            b'\r' | b'\n' => (LineEnding, Self::line_ending_len(b)),
            b' ' | b'\t' => (Whitespace, Self::prefix_len(b, |c| c == b' ' || c == b'\t')),
            c if c < b' ' || c == 127 => (
                Controls,
                Self::prefix_len(b, |c| {
                    c == 127 || (c < b' ' && c != b'\r' && c != b'\n' && c != b'\t')
                }),
            ),
            c if c.is_ascii_alphanumeric() || c == b'_' => (
                Symbol,
                Self::prefix_len(b, |c| c.is_ascii_alphanumeric() || c == b'_'),
            ),
            c => (Special(c), 1),
        }
    }

    /// Gets the prefix of {b} that matches {f}.
    pub fn prefix_len<F>(b: &[u8], f: F) -> usize
        where
            F: Fn(u8) -> bool,
    {
        b.iter().position(|c| !f(*c)).unwrap_or(b.len())
    }

    /// Gets the length of the line-ending token from the prefix of b.
    pub fn line_ending_len(b: &[u8]) -> usize {
        assert!(!b.is_empty());
        assert!(b[0] == b'\r' || b[0] == b'\n');

        if b[0] == b'\n' || b.len() == 1 || b[1] != b'\n' {
            1
        } else {
            2
        }
    }
}
