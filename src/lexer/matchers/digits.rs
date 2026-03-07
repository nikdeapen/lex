/// Matches one or more ASCII digits: `[0-9]+`.
pub fn digits(source: &str) -> Option<usize> {
    let bytes: &[u8] = source.as_bytes();
    let mut len: usize = 0;
    while len < bytes.len() && bytes[len].is_ascii_digit() {
        len += 1;
    }
    Some(len).filter(|len| *len > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_digits() {
        let test_cases: &[(&str, Option<usize>)] = &[
            ("", None),
            ("abc", None),
            ("123", Some(3)),
            ("123abc", Some(3)),
            ("0", Some(1)),
        ];

        for (source, expected) in test_cases {
            assert_eq!(digits(source), *expected, "source: {:?}", source);
        }
    }
}
