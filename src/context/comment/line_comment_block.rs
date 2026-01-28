use crate::{Context, Token};

impl<'a> Context<'a> {
    //! Line Comment Block

    /// Parses an optionally empty line-comment block.
    pub fn line_comment_block(&self) -> (Vec<Token<'a>>, Self) {
        let (comments, after_comments) = self.white_line_comments();
        if let Some(comments) = comments {
            let mut result: Vec<Token<'a>> = Vec::default();
            let (mut comments, _last) = comments.split_last_line();
            while !comments.is_empty() {
                let without_line_ending: Context = comments.without_ending_line_ending();
                let (not_last, last_line) = without_line_ending.split_last_line();
                comments = not_last;

                let (_white, last_line) = last_line.whitespace();
                if let (Some((_delimiter, comment, _line_ending)), _empty) =
                    last_line.line_comment()
                {
                    result.push(comment.token());
                } else {
                    break;
                }
            }
            (result, after_comments)
        } else {
            (vec![], after_comments)
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
            unsafe { self.split(last_cr_or_lf + 1) }
        } else {
            unsafe { self.split(0) }
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
        unsafe { self.split(self.len() - last_line_ending_len) }.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Context, Token};

    #[test]
    fn line_comment_block() {
        #[allow(clippy::useless_vec)]
        let text: String = vec![
            "  //ignored",
            "        ",
            "\t    //four",
            "    \t//three",
            "        //two",
            "\t\t//one",
            "    \t",
        ]
        .join("\n");

        let config: Config = unsafe { Config::default().with_line_comment_delimiter("//") };
        let context: Context = Context::new(Token::from(text.as_str()), &config);

        let (comments, _): (Vec<Token>, _) = context.line_comment_block();

        assert_eq!(
            comments.iter().map(|t| t.value()).collect::<Vec<&str>>(),
            vec!["one", "two", "three", "four"]
        );
    }
}
