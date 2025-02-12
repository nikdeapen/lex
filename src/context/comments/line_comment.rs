use crate::ParseContext;

impl<'a> ParseContext<'a> {
    //! Line Comments

    /// Parses an optional line-comment.
    ///
    /// Returns `(Some(delimiter, comment_line, line_ending), after_line_ending)`.
    /// Returns `(None, self)` if there is no line-comment.
    pub fn line_comment(&self) -> (Option<(Self, Self, Option<Self>)>, Self) {
        if let Some(line_comment_delimiter) =
            self.config().comment_config().line_comment_delimiter()
        {
            if self.value().starts_with(line_comment_delimiter) {
                let (delimiter, after_delimiter) =
                    unsafe { self.split(line_comment_delimiter.len()) };
                let (comment_line, line_ending, after_line_ending) = after_delimiter.rest_of_line();
                return (
                    Some((delimiter, comment_line, line_ending)),
                    after_line_ending,
                );
            }
        }
        (None, *self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{CommentConfig, Config, ParseContext, Token};

    #[test]
    fn line_comment() {
        let test_cases: &[(&str, Option<(&str, &str, Option<&str>)>, &str)] = &[
            ("", None, ""),
            ("/", None, "/"),
            (" //", None, " //"),
            ("//c", Some(("//", "c", None)), ""),
            ("//c\r", Some(("//", "c", Some("\r"))), ""),
            ("//c\r\n", Some(("//", "c", Some("\r\n"))), ""),
            ("//c\r\nafter", Some(("//", "c", Some("\r\n"))), "after"),
        ];

        let config: Config = unsafe {
            Config::default()
                .with_comment_config(CommentConfig::default().with_line_comment_delimiter("//"))
        };
        for (input, expected, after_expected) in test_cases {
            let c: ParseContext = ParseContext::new(Token::from(*input), &config);
            let (result, after_result) = c.line_comment();
            assert_eq!(after_result.value(), *after_expected);
            let result: Option<(&str, &str, Option<&str>)> =
                result.map(|(a, b, c)| (a.value(), b.value(), c.map(|c| c.value())));
            assert_eq!(result, *expected);
        }
    }
}
