/// Matches an exact keyword, only if not followed by an identifier character (`[a-zA-Z0-9_]`).
///
/// # Example
/// ```
/// use lex::keyword;
///
/// let matcher: fn(&str) -> Option<usize> = keyword!("message");
/// assert_eq!(matcher("message {"), Some(7));
/// assert_eq!(matcher("messageType"), None);
/// assert_eq!(matcher("message"), Some(7));
/// ```
#[macro_export]
macro_rules! keyword {
    ($s:literal) => {
        |source: &str| -> Option<usize> {
            if source.starts_with($s) {
                let len: usize = $s.len();
                if len == source.len() || {
                    let next: u8 = source.as_bytes()[len];
                    !next.is_ascii_alphanumeric() && next != b'_'
                } {
                    Some(len)
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn fn_keyword() {
        let matcher: fn(&str) -> Option<usize> = keyword!("message");

        let test_cases: &[(&str, Option<usize>)] = &[
            ("", None),
            ("msg", None),
            ("messageType", None),
            ("message_type", None),
            ("message123", None),
            ("message", Some(7)),
            ("message {", Some(7)),
            ("message\n", Some(7)),
        ];

        for (source, expected) in test_cases {
            assert_eq!(matcher(source), *expected, "source: {:?}", source);
        }
    }
}
