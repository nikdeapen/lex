use crate::Context;

impl<'a> Context<'a> {
    //! Whitespace

    /// Parses optional non-empty whitespace.
    ///
    /// Returns `(Some(whitespace), after_whitespace)`.
    /// Returns `(None, self)` when there is no whitespace.
    pub fn whitespace(&self) -> (Option<Self>, Self) {
        unsafe { self.match_prefix(|c| c == b' ' || c == b'\t') }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Context, Token};

    #[test]
    fn whitespace() {
        let test_cases: &[(&str, &str)] = &[
            ("text", ""),
            (" ", " "),
            ("\t", "\t"),
            (" \t", " \t"),
            (" text", " "),
            ("\ttext", "\t"),
            (" \ttext", " \t"),
        ];
        for (input, expected) in test_cases {
            let config: Config = Config::default();
            let parser: Context = Context::new(Token::from(*input), &config);
            let (result, _) = parser.whitespace();
            if expected.is_empty() {
                assert!(result.is_none());
            } else {
                assert_eq!(result.map(|p| p.token().value()), Some(*expected));
            }
        }
    }
}
