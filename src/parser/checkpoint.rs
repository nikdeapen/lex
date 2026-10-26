/// A parser checkpoint for backtracking.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Checkpoint {
    pos: usize,
}

impl Checkpoint {
    //! Construction

    /// Creates a new checkpoint.
    pub(in crate::parser) fn new(pos: usize) -> Self {
        Self { pos }
    }
}

impl Checkpoint {
    //! Properties

    /// Gets the token position.
    pub(in crate::parser) fn pos(self) -> usize {
        self.pos
    }
}
