use crate::Token;

impl<'a> Token<'a> {
    //! Split

    /// Checks if the `index` is a valid split index.
    ///
    /// Returns `true` if:
    ///     1. The index is a valid char boundary in `self.value()`.
    ///     2. The index does not split a `CRLF` char sequence.
    pub fn is_valid_split_index(&self, index: usize) -> bool {
        if index == 0 || index == self.value().len() {
            true
        } else if self.value().is_char_boundary(index) {
            let (a, b) = (
                self.value().as_bytes()[index - 1],
                self.value().as_bytes()[index],
            );
            !(a == b'\r' && b == b'\n')
        } else {
            false
        }
    }

    /// Splits the token at the `index`.
    ///
    /// Returns `(before_index, index_and_after)`.
    ///
    /// # Safety
    /// The `index` must be a valid split index.
    pub unsafe fn split(&self, index: usize) -> (Self, Self) {
        debug_assert!(self.is_valid_split_index(index));

        let (left, right) = self.value().split_at(index);
        let left: Self = Self::new(left, self.line(), self.position());
        let (line_ending_count, last_line_len) = left.line_ending_count_and_last_line_len();
        let right: Self = Self::new(
            right,
            self.line() + line_ending_count,
            if line_ending_count == 0 {
                self.position() + last_line_len
            } else {
                last_line_len
            },
        );
        (left, right)
    }

    /// Splits the token at the `index`.
    ///
    /// Returns `(Some(before_index), index_and_after)` if the `index` is not `0`.
    /// Returns `(None, self)` if the `index` is `0`.
    ///
    /// # Safety
    /// The `index` must be a valid split index.
    pub unsafe fn split_optional(&self, index: usize) -> (Option<Self>, Self) {
        debug_assert!(self.is_valid_split_index(index));

        if index == 0 {
            (None, *self)
        } else {
            let (left, right) = self.split(index);
            (Some(left), right)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Token;

    #[test]
    fn is_valid_split_index() {
        let test_cases: &[(&str, usize, bool)] = &[
            ("", 0, true),
            ("\r\n", 0, true),
            ("\r\n", 2, true),
            ("你好", 0, true),
            ("你好", 1, false),
            ("你好", 2, false),
            ("你好", 3, true),
            ("你好", 4, false),
            ("你好", 5, false),
            ("你好", 6, true),
            ("\r\n", 1, false),
            ("\rx", 1, true),
            ("x\n", 1, true),
            ("\n\r", 1, true),
        ];
        for (input, index, expected) in test_cases {
            let result: bool = Token::from(*input).is_valid_split_index(*index);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn split() {
        let test_cases: &[(&str, usize, &str, Token)] = &[
            ("", 0, "", Token::from("")),
            ("hello", 0, "", Token::from("hello")),
            ("hello", 5, "hello", Token::new("", 0, 5)),
            ("0\r1\n2", 2, "0\r", Token::new("1\n2", 1, 0)),
            ("0\r1\n2", 4, "0\r1\n", Token::new("2", 2, 0)),
            ("0\r1\n2\r\n123", 7, "0\r1\n2\r\n", Token::new("123", 3, 0)),
            ("0\r1\n2\r\n123", 10, "0\r1\n2\r\n123", Token::new("", 3, 3)),
        ];
        for (input, index, expected_left, expected_right) in test_cases {
            let (left, right) = unsafe { Token::from(*input).split(*index) };
            assert_eq!(left.value(), *expected_left);
            assert_eq!(right, *expected_right);
        }
    }
}
