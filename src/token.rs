use std::fmt::{Display, Formatter};

use crate::Kind;

/// A lexical token.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Token<'a> {
    kind: Kind,
    value: &'a str,
    line: u32,
    position: u32,
}

impl<'a> Token<'a> {
    //! Construction

    /// Creates a new token.
    pub fn new(kind: Kind, value: &'a str, line: u32, position: u32) -> Self {
        debug_assert!(kind.is_valid(value), "kind={} value={}", kind, value);

        Self {
            kind,
            value,
            line,
            position,
        }
    }
}

impl<'a> Token<'a> {
    //! Properties

    /// Gets the kind of token.
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Gets the raw value.
    pub fn value(&self) -> &str {
        self.value
    }

    /// Gets the 0-indexed line number.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Gets the 0-indexed position of the first byte within the line.
    pub fn position(&self) -> u32 {
        self.position
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token([{}:{}]:{} ", self.line, self.position, self.kind)?;
        for c in self.value.as_bytes() {
            write!(f, "{}", Kind::display_byte(*c))?;
        }
        write!(f, ")")
    }
}
