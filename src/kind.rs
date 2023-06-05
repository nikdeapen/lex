use std::fmt::{Display, Formatter};

/// A kind of lexical token.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub enum Kind {
    /// A sequence of non US-ASCII code points. (code points above 127)
    NonAscii,

    /// A single US-ASCII line-ending. (CR, LF, or CRLF)
    LineEnding,

    /// A sequence of US-ASCII whitespace chars. (spaces & tabs)
    Whitespace,

    /// A sequence of US-ASCII control chars. (excluding CR, LF, & TAB)
    Controls,

    /// A sequence of US-ASCII letters, numbers, & underscores.
    Symbol,

    /// A single special US-ASCII char. (punctuation excluding underscores)
    Special(u8),
}

impl Kind {
    //! Validation

    /// Checks if the value is valid for the kind of token.
    pub fn is_valid(&self, value: &str) -> bool {
        match self {
            Self::NonAscii => value.as_bytes().iter().all(|c| *c > 127),
            Self::LineEnding => value == "\r" || value == "\n" || value == "\r\n",
            Self::Controls => value
                .as_bytes()
                .iter()
                .all(|c| *c == 127 || (*c < b' ' && *c != b'\t' && *c != b'\r' && *c != b'\n')),
            Self::Whitespace => value.as_bytes().iter().all(|c| *c == b' ' || *c == b'\t'),
            Self::Symbol => value
                .as_bytes()
                .iter()
                .all(|c| c.is_ascii_alphanumeric() || *c == b'_'),
            Self::Special(c) => {
                value.len() == 1
                    && *c == value.as_bytes()[0]
                    && c.is_ascii_punctuation()
                    && *c != b'_'
            }
        }
    }
}

impl Kind {
    //! Display

    /// Gets the displayable char for the byte.
    pub const fn display_byte(b: u8) -> char {
        match b {
            b'\r' => 'r',
            b'\n' => 'n',
            b' ' => 's',
            b'\t' => 't',
            c if c.is_ascii_alphanumeric() => c as char,
            c if c.is_ascii_punctuation() => c as char,
            _ => '?',
        }
    }

    /// Converts the kind to a static string.
    pub const fn to_str(&self) -> &'static str {
        match self {
            Self::NonAscii => "non-ascii",
            Self::LineEnding => "line-ending",
            Self::Whitespace => "whitespace",
            Self::Controls => "controls",
            Self::Symbol => "symbol",
            Self::Special(_) => "special",
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
