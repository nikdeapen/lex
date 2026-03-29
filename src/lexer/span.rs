use std::fmt::{Display, Formatter};

/// A byte span in source text.
///
/// The `offset` and `offset + len` must lie on valid UTF-8 char boundaries in the source text the
/// span was derived from. The `text` function will panic otherwise.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Span {
    offset: u32,
    len: u32,
}

impl Span {
    //! Construction

    /// Creates a new span.
    pub const fn new(offset: u32, len: u32) -> Self {
        debug_assert!(offset.checked_add(len).is_some());

        Self { offset, len }
    }
}

impl Span {
    //! Properties

    /// Gets the byte offset. (0-indexed)
    pub const fn offset(self) -> u32 {
        self.offset
    }

    /// Gets the number of bytes.
    pub const fn len(self) -> u32 {
        self.len
    }

    /// Gets the end byte offset. (`offset + len`)
    pub const fn end(self) -> u32 {
        self.offset + self.len
    }

    /// Checks if the span is empty.
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }
}

impl Span {
    //! Line & Column

    /// Gets the 0-indexed line and column for the start of the span.
    pub fn line_column(self, source: &str) -> (usize, usize) {
        debug_assert!(source.is_char_boundary(self.offset as usize));

        let offset: usize = self.offset as usize;
        let mut line: usize = 0;
        let mut col: usize = 0;
        for byte in &source.as_bytes()[..offset] {
            if *byte == b'\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
}

impl Span {
    //! Text

    /// Gets the text from the `source`.
    pub fn text(self, source: &str) -> &str {
        debug_assert!(source.is_char_boundary(self.offset as usize));
        debug_assert!(source.is_char_boundary(self.end() as usize));

        let start: usize = self.offset as usize;
        let end: usize = start + self.len as usize;
        &source[start..end]
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "span[offset={}, len={}]", self.offset, self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text() {
        let source: &str = "hello world";
        assert_eq!(Span::new(0, 5).text(source), "hello");
        assert_eq!(Span::new(6, 5).text(source), "world");
        assert_eq!(Span::new(0, 0).text(source), "");
        assert_eq!(Span::new(0, 11).text(source), "hello world");
    }

    #[test]
    fn line_column() {
        let source: &str = "ab\ncd\nef";
        assert_eq!(Span::new(0, 1).line_column(source), (0, 0));
        assert_eq!(Span::new(1, 1).line_column(source), (0, 1));
        assert_eq!(Span::new(3, 1).line_column(source), (1, 0));
        assert_eq!(Span::new(4, 1).line_column(source), (1, 1));
        assert_eq!(Span::new(6, 1).line_column(source), (2, 0));
        assert_eq!(Span::new(7, 1).line_column(source), (2, 1));
        assert_eq!(Span::new(0, 0).line_column(""), (0, 0));
        assert_eq!(Span::new(0, 1).line_column("a"), (0, 0));
    }
}
