/// Matches one or more ASCII whitespace bytes: spaces, tabs, newlines, and carriage returns.
pub fn whitespace(source: &str) -> Option<usize> {
    let bytes: &[u8] = source.as_bytes();
    let mut len: usize = 0;
    while len < bytes.len() && bytes[len].is_ascii_whitespace() {
        len += 1;
    }
    Some(len).filter(|len| *len > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_whitespace() {
        let test_cases: &[(&str, Option<usize>)] = &[
            ("", None),
            ("abc", None),
            (" ", Some(1)),
            ("   ", Some(3)),
            ("\t", Some(1)),
            ("\n", Some(1)),
            ("\r\n", Some(2)),
            (" \t\n", Some(3)),
            ("  abc", Some(2)),
        ];

        for (source, expected) in test_cases {
            assert_eq!(whitespace(source), *expected, "source: {:?}", source);
        }
    }
}
