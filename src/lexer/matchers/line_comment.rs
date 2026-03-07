/// Matches a line comment from the delimiter to the end of the line (inclusive of newline).
#[macro_export]
macro_rules! line_comment {
    ($delim:literal) => {
        |source: &str| -> Option<usize> {
            if !source.starts_with($delim) {
                return None;
            }
            let len: usize = match source[$delim.len()..].find('\n') {
                Some(pos) => $delim.len() + pos + 1,
                None => source.len(),
            };
            Some(len)
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn fn_line_comment() {
        let matcher: fn(&str) -> Option<usize> = line_comment!("//");

        let test_cases: &[(&str, Option<usize>)] = &[
            ("", None),
            ("hello", None),
            ("// comment\n", Some(11)),
            ("// comment\nnext", Some(11)),
            ("// no newline", Some(13)),
            ("//\n", Some(3)),
            ("//", Some(2)),
        ];

        for (source, expected) in test_cases {
            assert_eq!(matcher(source), *expected, "source: {:?}", source);
        }
    }
}
