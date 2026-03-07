/// Matches an identifier: `[a-zA-Z_][a-zA-Z0-9_]*`.
pub fn ident(source: &str) -> Option<usize> {
    let bytes: &[u8] = source.as_bytes();
    if bytes.is_empty() || (!bytes[0].is_ascii_alphabetic() && bytes[0] != b'_') {
        return None;
    }
    let mut len: usize = 1;
    while len < bytes.len() && (bytes[len].is_ascii_alphanumeric() || bytes[len] == b'_') {
        len += 1;
    }
    Some(len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_ident() {
        let test_cases: &[(&str, Option<usize>)] = &[
            ("", None),
            ("123", None),
            ("foo", Some(3)),
            ("foo123", Some(6)),
            ("_foo", Some(4)),
            ("foo bar", Some(3)),
            ("x", Some(1)),
            ("_", Some(1)),
        ];

        for (source, expected) in test_cases {
            assert_eq!(ident(source), *expected, "source: {:?}", source);
        }
    }
}
