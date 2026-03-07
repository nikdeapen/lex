/// Matches an exact string literal.
///
/// # Example
/// ```
/// use lex::literal;
///
/// let matcher: fn(&str) -> Option<usize> = literal!("==");
/// assert_eq!(matcher("== 1"), Some(2));
/// assert_eq!(matcher("!="), None);
/// ```
#[macro_export]
macro_rules! literal {
    ($s:literal) => {
        |source: &str| -> Option<usize> { source.starts_with($s).then_some($s.len()) }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn fn_literal() {
        let matcher: fn(&str) -> Option<usize> = literal!("==");

        let test_cases: &[(&str, Option<usize>)] = &[
            ("", None),
            ("!=", None),
            ("=", None),
            ("==", Some(2)),
            ("== 1", Some(2)),
            ("===", Some(2)),
        ];

        for (source, expected) in test_cases {
            assert_eq!(matcher(source), *expected, "source: {:?}", source);
        }
    }
}
