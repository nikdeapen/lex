use std::fmt::{Display, Formatter};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

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
    // Combining

    /// Combines two adjacent or overlapping spans into a span covering both.
    #[must_use]
    pub fn combine(self, other: Span) -> Span {
        debug_assert!(self.end() >= other.offset || other.end() >= self.offset);

        let offset: u32 = self.offset.min(other.offset);
        let end: u32 = self.end().max(other.end());
        Span::new(offset, end - offset)
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
    // Line & Column

    /// Gets the 0-indexed line and column for the start of the span.
    ///
    /// The column is measured in display width using [UnicodeWidthChar].
    pub fn line_column(self, source: &str) -> (usize, usize) {
        debug_assert!(source.is_char_boundary(self.offset as usize));

        let prefix: &str = &source[..self.offset as usize];
        let mut line: usize = 0;
        let mut col: usize = 0;
        for c in prefix.chars() {
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += c.width().unwrap_or(0);
            }
        }
        (line, col)
    }
}

impl Span {
    // Char Count & Display Width

    /// Gets the number of characters in the span.
    pub fn char_count(self, source: &str) -> usize {
        self.text(source).chars().count()
    }

    /// Gets the display width of the span's text using [UnicodeWidthStr].
    pub fn display_width(self, source: &str) -> usize {
        self.text(source).width()
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

    /// Gets the text of the line containing the start of the span.
    pub fn line_text(self, source: &str) -> &str {
        let offset: usize = self.offset as usize;
        let start: usize = source[..offset].rfind('\n').map_or(0, |i| i + 1);
        let end: usize = source[offset..]
            .find('\n')
            .map_or(source.len(), |i| offset + i);
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

    // Combining

    #[test]
    fn combine_adjacent() {
        assert_eq!(Span::new(0, 3).combine(Span::new(3, 3)), Span::new(0, 6));
    }

    #[test]
    fn combine_overlapping() {
        assert_eq!(Span::new(0, 5).combine(Span::new(3, 5)), Span::new(0, 8));
    }

    #[test]
    fn combine_same() {
        assert_eq!(Span::new(3, 4).combine(Span::new(3, 4)), Span::new(3, 4));
    }

    #[test]
    fn combine_reversed() {
        assert_eq!(Span::new(5, 3).combine(Span::new(0, 3)), Span::new(0, 8));
    }

    // Line & Column

    #[test]
    fn line_column() {
        let source: &str = "ab\ncd\nef";
        assert_eq!(Span::new(0, 1).line_column(source), (0, 0));
        assert_eq!(Span::new(1, 1).line_column(source), (0, 1));
        assert_eq!(Span::new(3, 1).line_column(source), (1, 0));
        assert_eq!(Span::new(4, 1).line_column(source), (1, 1));
        assert_eq!(Span::new(6, 1).line_column(source), (2, 0));
        assert_eq!(Span::new(7, 1).line_column(source), (2, 1));
    }

    #[test]
    fn line_column_empty() {
        assert_eq!(Span::new(0, 0).line_column(""), (0, 0));
    }

    #[test]
    fn line_column_multibyte() {
        assert_eq!(Span::new(3, 2).line_column("café"), (0, 3));
    }

    #[test]
    fn line_column_cjk() {
        let source: &str = "你好";
        assert_eq!(Span::new(0, 3).line_column(source), (0, 0));
        assert_eq!(Span::new(3, 3).line_column(source), (0, 2));
    }

    #[test]
    fn line_column_mixed() {
        let source: &str = "a你b\nc";
        assert_eq!(Span::new(4, 1).line_column(source), (0, 3));
        assert_eq!(Span::new(6, 1).line_column(source), (1, 0));
    }

    // Char Count & Display Width

    #[test]
    fn char_count_ascii() {
        assert_eq!(Span::new(0, 5).char_count("hello"), 5);
    }

    #[test]
    fn char_count_empty() {
        assert_eq!(Span::new(0, 0).char_count("hello"), 0);
    }

    #[test]
    fn char_count_multibyte() {
        assert_eq!(Span::new(0, 5).char_count("café!"), 4);
    }

    #[test]
    fn char_count_cjk() {
        assert_eq!(Span::new(0, 6).char_count("你好"), 2);
    }

    #[test]
    fn display_width_ascii() {
        assert_eq!(Span::new(0, 5).display_width("hello"), 5);
    }

    #[test]
    fn display_width_empty() {
        assert_eq!(Span::new(0, 0).display_width("hello"), 0);
    }

    #[test]
    fn display_width_cjk() {
        assert_eq!(Span::new(0, 6).display_width("你好"), 4);
    }

    #[test]
    fn display_width_multibyte() {
        assert_eq!(Span::new(0, 5).display_width("café!"), 4);
    }

    // Text

    #[test]
    fn text() {
        let source: &str = "hello world";
        assert_eq!(Span::new(0, 5).text(source), "hello");
        assert_eq!(Span::new(6, 5).text(source), "world");
        assert_eq!(Span::new(0, 0).text(source), "");
        assert_eq!(Span::new(0, 11).text(source), "hello world");
    }

    #[test]
    fn line_text_multiline() {
        let source: &str = "abc\ndef\nghi";
        assert_eq!(Span::new(0, 1).line_text(source), "abc");
        assert_eq!(Span::new(2, 1).line_text(source), "abc");
        assert_eq!(Span::new(4, 1).line_text(source), "def");
        assert_eq!(Span::new(8, 1).line_text(source), "ghi");
    }

    #[test]
    fn line_text_single_line() {
        assert_eq!(Span::new(0, 5).line_text("hello"), "hello");
    }

    #[test]
    fn line_text_last_line() {
        assert_eq!(Span::new(4, 3).line_text("abc\ndef"), "def");
    }
}
