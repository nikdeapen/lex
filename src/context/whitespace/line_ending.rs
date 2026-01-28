use crate::{Context, Token};

impl<'a> Context<'a> {
    //! Line Ending

    /// Parses an optional line-ending.
    ///
    /// Returns `(Some(line_ending), after_line_ending)`.
    /// Returns `(None, self)` when there is no line-ending.
    pub fn line_ending(&self) -> (Option<Self>, Self) {
        let line_ending_len: usize = Token::line_ending_prefix_len(self.value());
        unsafe { self.split_optional(line_ending_len) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Context, Token};

    #[test]
    fn line_ending() {
        let test_cases: &[(&str, &str)] = &[
            ("", ""),
            ("\r", "\r"),
            ("\n", "\n"),
            ("\r\n", "\r\n"),
            ("\rtext", "\r"),
            ("\ntext", "\n"),
            ("\r\ntext", "\r\n"),
            ("\n\r", "\n"),
        ];
        for (input, expected) in test_cases {
            let config: Config = Config::default();
            let parser: Context = Context::new(Token::from(*input), &config);
            let (result, _) = parser.line_ending();
            if expected.is_empty() {
                assert!(result.is_none());
            } else {
                assert_eq!(result.map(|p| p.token().value()), Some(*expected),);
            }
        }
    }
}
