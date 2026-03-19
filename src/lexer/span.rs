/// A byte span in source text.
///
/// The `offset` and `offset + len` must lie on valid UTF-8 char boundaries in the source text the
/// span was derived from. The `text` function will panic otherwise.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
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

    /// Checks if the span is empty.
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }
}

impl Span {
    //! Text

    /// Gets the text from the `source`.
    pub fn text<'src>(self, source: &'src str) -> &'src str {
        debug_assert!(source.len() <= u32::MAX as usize);

        let start: usize = self.offset as usize;
        let end: usize = start + self.len as usize;
        &source[start..end]
    }
}
