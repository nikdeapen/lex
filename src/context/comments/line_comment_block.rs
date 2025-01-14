use crate::{ParseContext, Token};

impl<'a> ParseContext<'a> {
    //! Line Comment Block

    /// Parses the line-comment block vec.
    ///
    /// Returns the line-comments as a vec of `(delimiter, line_text)`.
    pub fn line_comment_block_vec(self) -> Vec<Token<'a>> {
        let mut result: Vec<Token> = Vec::default();
        self.line_comment_block(|comment| result.push(comment.clone()));
        result
    }

    /// Parses a line-comment block.
    ///
    /// Adds the line-comments as `(delimiter, line_text)`.
    pub fn line_comment_block<F>(self, mut add_fn: F)
    where
        F: FnMut(Token<'a>),
    {
        let line_comment_delimiter: &str =
            if let Some(lcd) = self.config().comment_config().line_comment_delimiter() {
                lcd
            } else {
                return;
            };

        let (mut lines, last_line) = self.split_last_line();
        let last_line_indent: usize = last_line.indent_level_and_len().0;

        while !lines.is_empty() {
            let without_line_ending: ParseContext = lines.without_ending_line_ending();
            let (not_last_line, last_line) = without_line_ending.split_last_line();
            lines = not_last_line;

            let (indent, len) = last_line.indent_level_and_len();
            if indent != last_line_indent {
                return;
            }

            let last_line: Token = unsafe { last_line.split_unchecked(len) }.1.token();
            if last_line.value().starts_with(line_comment_delimiter) {
                add_fn(unsafe { last_line.split_unchecked(line_comment_delimiter.len()) }.1);
            }
        }
    }

    /// Splits off the last line.
    fn split_last_line(&self) -> (Self, Self) {
        if let Some(last_cr_or_lf) = self
            .value()
            .as_bytes()
            .iter()
            .rposition(|c| *c == b'\r' || *c == b'\n')
        {
            unsafe { self.split_unchecked(last_cr_or_lf + 1) }
        } else {
            unsafe { self.split_unchecked(0) }
        }
    }

    /// Gets the context without the ending line-ending.
    ///
    /// # Examples
    /// `abc\r\n` -> `abc`
    /// `abc\n\r` -> `abc\n`
    /// `abc\nab` -> `abc\nab`
    fn without_ending_line_ending(&self) -> Self {
        let last_line_ending_len: usize = if self.is_empty() {
            0
        } else {
            let last: u8 = self.value().as_bytes()[self.len() - 1];
            if last == b'\r' {
                1
            } else if last == b'\n' {
                if self.len() == 1 {
                    1
                } else {
                    let before_last: u8 = self.value().as_bytes()[self.len() - 2];
                    if before_last == b'\r' {
                        2
                    } else {
                        1
                    }
                }
            } else {
                0
            }
        };
        unsafe { self.split_unchecked(self.len() - last_line_ending_len) }.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{CommentConfig, Config, ParseContext, Token};

    #[test]
    fn line_comment_block() {
        let text: String = vec![
            "ignored",
            "        /ignored",
            "\t    //four",
            "    \t//three",
            "        //two",
            "\t\t//one",
            "    \t",
        ]
        .join("\n");

        let config: Config = unsafe {
            Config::default().with_comment_config(
                CommentConfig::default().with_line_comment_delimiter_unchecked("//"),
            )
        };
        let context: ParseContext = ParseContext::new(Token::from(text.as_str()), &config);

        let comments: Vec<Token> = context.line_comment_block_vec();

        assert_eq!(
            comments.iter().map(|t| t.value()).collect::<Vec<&str>>(),
            vec!["one", "two", "three", "four"]
        );
    }
}
