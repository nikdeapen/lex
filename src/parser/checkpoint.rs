/// A parser checkpoint for backtracking.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Checkpoint {
    pos: usize,
    error_count: usize,
}

impl Checkpoint {
    //! Construction

    /// Creates a new checkpoint.
    pub(in crate::parser) fn new(pos: usize, error_count: usize) -> Self {
        Self { pos, error_count }
    }
}

impl Checkpoint {
    //! Properties

    /// Gets the token position.
    pub(in crate::parser) fn pos(self) -> usize {
        self.pos
    }

    /// Gets the error count.
    pub(in crate::parser) fn error_count(self) -> usize {
        self.error_count
    }
}
