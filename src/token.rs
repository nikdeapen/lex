use std::fmt::{Display, Formatter};

/// A lexical token.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct Token<'a> {
    line: u32,
    position: u32,
    value: &'a str,
}

impl<'a> Token<'a> {
    //! Construction

    /// Creates a new token.
    fn new(line: u32, position: u32, value: &'a str) -> Self {
        debug_assert!(value.len() <= (u32::MAX as usize));

        Self {
            line,
            position,
            value,
        }
    }
}

impl<'a> TryFrom<&'a str> for Token<'a> {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value.len() > (u32::MAX as usize) {
            Err("token value length >= 4 GiB")
        } else {
            Ok(Token::new(0, 0, value))
        }
    }
}

impl<'a> Token<'a> {
    //! Properties

    /// Gets the 0-indexed line number.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Gets the 0-indexed position within the line.
    pub fn position(&self) -> u32 {
        self.position
    }

    /// Gets the value.
    pub fn value(&self) -> &str {
        self.value
    }
}

impl<'a> Token<'a> {
    //! Derived Properties

    /// Gets the value as bytes.
    pub fn bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    /// Gets the length (in bytes) of the value.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Checks if the value is empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Checks if the token is a line-ending.
    pub fn is_line_ending(&self) -> bool {
        self.value == "\r" || self.value == "\n" || self.value == "\r\n"
    }

    /// Checks if the token is whitespace.
    pub fn is_whitespace(&self) -> bool {
        self.bytes().iter().all(|c| *c == b' ' || *c == b'\t')
    }
}

impl<'a> Token<'a> {
    //! Mutations

    /// Increments the line and resets the position.
    pub fn with_new_line(mut self) -> Self {
        self.line += 1;
        self.position = 0;
        self
    }
}

impl<'a> Token<'a> {
    //! Split

    /// Splits `prefix_len` bytes from the token prefix.
    pub fn split(&self, prefix_len: usize) -> (Token<'a>, Token<'a>) {
        debug_assert!(self.value.is_char_boundary(prefix_len));

        let (a, b) = self.value.split_at(prefix_len);
        (
            Token::new(self.line, self.position, a),
            Token::new(self.line, self.position + (prefix_len as u32), b),
        )
    }

    /// Splits `prefix_len` bytes from the token prefix.
    /// - Returns `None` if the `prefix_len` is 0.
    pub fn split_optional(&self, prefix_len: usize) -> (Option<Token<'a>>, Token<'a>) {
        debug_assert!(self.value.is_char_boundary(prefix_len));

        if prefix_len == 0 {
            (None, *self)
        } else {
            let (a, b) = self.split(prefix_len);
            (Some(a), b)
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[line={} pos={} len={}]: ",
            self.line,
            self.position,
            self.len()
        )?;
        if self.is_line_ending() {
            write!(f, "line-ending(")?;
            for c in self.value.as_bytes() {
                if *c == b'\r' {
                    write!(f, "r")?;
                } else {
                    write!(f, "n")?;
                }
            }
            write!(f, ")")
        } else if self.is_whitespace() {
            let len: usize = self
                .bytes()
                .iter()
                .map(|c| if *c == b' ' { 1 } else { 4 })
                .sum();
            write!(f, "whitespace(spaces={})", len)
        } else {
            write!(f, "{}", self.value)
        }
    }
}
