use crate::Context;

impl<'a> Context<'a> {
    //! White Line Comments

    /// Parses optional non-empty whitespace, line-endings, & line-comments.
    ///
    /// Returns `(Some(white_line_comments), after_white_line_comments)`.
    /// Returns `(None, self)` if there is no whitespace, line-endings, or line-comments.
    pub fn white_line_comments(&self) -> (Option<Self>, Self) {
        let mut lex: Self = *self;
        let mut len: usize = 0;
        loop {
            let mut matched: bool = false;

            if let (Some(white_lines), after_white_lines) = lex.white_lines() {
                matched = true;
                len += white_lines.len();
                lex = after_white_lines;
            }

            if let (Some((delimiter, comment_line, line_ending)), after_white_lines) =
                lex.line_comment()
            {
                matched = true;
                len += delimiter.len()
                    + comment_line.len()
                    + line_ending.map(|le| le.len()).unwrap_or(0);
                lex = after_white_lines;
            }

            if !matched {
                return unsafe { self.split_optional(len) };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Context, Token};

    #[test]
    fn white_line_comments() {
        let test_cases: &[(&str, &str, &str)] = &[
            ("", "", ""),
            ("x", "", "x"),
            (" \r\n x", " \r\n ", "x"),
            (" \r\n //x", " \r\n //x", ""),
            (" \r\n //x\r\nx", " \r\n //x\r\n", "x"),
        ];

        let config: Config = unsafe { Config::default().with_line_comment_delimiter("//") };
        for (input, expected, after_expected) in test_cases {
            let c: Context = Context::new(Token::from(*input), &config);
            let (result, after_result) = c.white_line_comments();
            assert_eq!(result.map(|r| r.value()).unwrap_or(""), *expected);
            assert_eq!(after_result.value(), *after_expected);
        }
    }
}
