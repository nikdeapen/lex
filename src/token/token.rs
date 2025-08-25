use std::fmt::{Debug, Display, Formatter};

/// A string value with an associated source code position.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Token<'a> {
    value: &'a str,
    line: usize,
    position: usize,
}

impl<'a> Token<'a> {
    //! Construction

    /// Creates a new token.
    #[inline(always)]
    pub const fn new(value: &'a str, line: usize, position: usize) -> Self {
        Self {
            value,
            line,
            position,
        }
    }
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        Self::new(value, 0, 0)
    }
}

impl<'a> Token<'a> {
    //! Properties

    /// Gets the string value.
    pub fn value(&self) -> &'a str {
        self.value
    }

    /// Gets the 0-indexed line number.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Gets the 0-indexed byte position of the first byte within the line.
    pub fn position(&self) -> usize {
        self.position
    }
}

impl<'a> Token<'a> {
    //! Derived Properties

    /// Gets the length of the string value. (in bytes)
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Checks if the string value is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

impl<'a> Debug for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[v=")?;
        for c in self.value.chars() {
            let c: char = match c {
                c if c.is_ascii_control() => '🅒',
                c if c.is_ascii() => c,
                _ => '🅧',
            };
            write!(f, "{}", c)?;
        }
        write!(f, " line={} pos={}]", self.line, self.position)
    }
}
