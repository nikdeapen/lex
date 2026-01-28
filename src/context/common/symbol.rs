use crate::{Context, Token};

impl<'a> Context<'a> {
    //! Symbols

    /// Parses an optional non-empty symbol.
    ///
    /// Returns `(Some(symbol), after_symbol)`.
    /// Returns `(None, self)` if there is no symbol.
    pub fn symbol(&self) -> (Option<Token<'a>>, Self) {
        if let (Some(symbol), after_symbol) =
            unsafe { self.match_prefix(|c| c.is_ascii_alphanumeric() || c == b'_') }
        {
            (Some(symbol.token()), after_symbol)
        } else {
            (None, *self)
        }
    }

    /// Parses an optional exact symbol.
    ///
    /// This is different from `exact()` in that it ensures the `symbol` is the full symbol and not
    /// just the prefix of the symbol.
    ///
    /// Returns `(Some(symbol), after_symbol)`.
    /// Returns `(None, self)` when the `symbol` is not present.
    pub fn exact_symbol(&self, symbol: &str) -> (Option<Token<'a>>, Self) {
        if let (Some(s), after_symbol) = self.symbol() {
            if s.value() == symbol {
                return (Some(s), after_symbol);
            }
        }
        (None, *self)
    }
}
