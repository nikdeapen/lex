use std::fmt::{Display, Formatter};

use crate::Kind::{LineEnding, Special};
use crate::{Kind, Token};

/// Tokenized source code.
#[derive(Clone, Debug)]
pub struct Lex<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
}

impl<'a> Lex<'a> {
    //! Construction

    /// Creates a new lex.
    pub(crate) fn new(source: &'a str, tokens: Vec<Token<'a>>) -> Self {
        debug_assert!(Self::matches(source, tokens.as_slice()));

        Self { source, tokens }
    }
}

impl<'a> Lex<'a> {
    //! Validation

    /// Checks if the tokens match the source code.
    fn matches(source: &'a str, tokens: &[Token<'a>]) -> bool {
        Self::matches_source(source, tokens)
            && Self::matches_lines_and_positions(tokens)
            && Self::has_no_duplicates(tokens)
    }

    /// Checks if the source matches the tokens values.
    fn matches_source(source: &'a str, tokens: &[Token<'a>]) -> bool {
        let mut remaining: &str = source;
        for token in tokens {
            if remaining.len() < token.value().len() {
                return false;
            } else if &remaining[..token.value().len()] != token.value() {
                return false;
            }
            remaining = &remaining[token.value().len()..];
        }
        remaining.len() == 0
    }

    /// Checks if there no duplicate tokens.
    fn has_no_duplicates(tokens: &[Token<'a>]) -> bool {
        if tokens.len() < 2 {
            true
        } else {
            let mut last: &Token = &tokens[0];
            for token in &tokens[1..] {
                if token.kind() == last.kind() {
                    if matches!(token.kind(), Special(_)) {
                        last = token;
                        continue;
                    } else if token.kind() != LineEnding {
                        return false;
                    } else if last.value() == "\r" && token.value() == "\n" {
                        return false;
                    } else {
                        last = token;
                        continue;
                    }
                }
                last = token;
            }
            true
        }
    }

    /// Checks if the lines and positions are correct.
    fn matches_lines_and_positions(tokens: &[Token]) -> bool {
        let mut line: u32 = 0;
        let mut position: u32 = 0;
        for token in tokens {
            if line != token.line() || position != token.position() {
                return false;
            }
            if token.kind() == LineEnding {
                if let Some(l) = line.checked_add(1) {
                    line = l;
                } else {
                    return false;
                }
                position = 0;
            } else {
                if token.value().len() > u32::MAX as usize {
                    return false;
                }
                let len: u32 = token.value().len() as u32;
                if let Some(p) = position.checked_add(len) {
                    position = p;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

impl<'a> Lex<'a> {
    //! Properties

    /// Gets the source code.
    pub const fn source(&self) -> &str {
        self.source
    }

    /// Gets the tokens.
    pub fn tokens(&self) -> &[Token<'a>] {
        self.tokens.as_slice()
    }
}

impl<'a> Display for Lex<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut line: u32 = 0;
        for token in self.tokens.as_slice() {
            if line == token.line() {
                line += 1;
                write!(f, "line {}:\n", line)?;
            }
            write!(
                f,
                "    {:<5}: {:<16} ",
                token.position(),
                token.kind().to_str()
            )?;
            for b in token.value().as_bytes() {
                write!(f, "{}", Kind::display_byte(*b))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
