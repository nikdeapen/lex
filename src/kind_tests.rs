use crate::Kind;

#[test]
fn is_valid() {
    let test_cases: &[(Kind, &str, bool)] = &[
        (Kind::NonAscii, "你好", true),
        (Kind::NonAscii, "\x7F", false),
        (Kind::LineEnding, "\r", true),
        (Kind::LineEnding, "\n", true),
        (Kind::LineEnding, "\r\n", true),
        (Kind::LineEnding, "\r\r", false),
        (Kind::LineEnding, "\n\n", false),
        (Kind::LineEnding, "\r ", false),
        (Kind::LineEnding, "\n ", false),
        (Kind::LineEnding, "\n\r", false),
        (Kind::Whitespace, " \t \t", true),
        (Kind::Whitespace, " \t \t\r", false),
        (Kind::Controls, "\x00\x1F\x7F", true),
        (Kind::Controls, " ", false),
        (Kind::Controls, "\r", false),
        (Kind::Controls, "\n", false),
        (Kind::Controls, "\t", false),
        (Kind::Controls, "~", false),
        (Kind::Symbol, "azAZ09_", true),
        (Kind::Symbol, "/", false),
        (Kind::Symbol, ":", false),
        (Kind::Symbol, "@", false),
        (Kind::Symbol, "[", false),
        (Kind::Symbol, "`", false),
        (Kind::Symbol, "{", false),
        (Kind::Special(b'~'), "~", true),
        (Kind::Special(b'~'), "!", false),
        (Kind::Special(b'a'), "a", false),
    ];
    for (kind, value, expected) in test_cases {
        let result: bool = kind.is_valid(*value);
        assert_eq!(result, *expected, "kind={}, value={}", kind, value);
    }
}

#[test]
fn display_byte() {
    let test_cases: &str = " \t\r\nazAZ09:!你好";
    let expected: &str = "strnazAZ09:!??????";
    assert_eq!(test_cases.len(), expected.len());

    for (t, e) in test_cases.as_bytes().iter().zip(expected.as_bytes()) {
        let result: char = Kind::display_byte(*t);
        assert_eq!(*e as char, result);
    }
}

#[test]
fn display() {
    assert_eq!(Kind::NonAscii.to_string(), "non-ascii");
    assert_eq!(Kind::LineEnding.to_string(), "line-ending");
    assert_eq!(Kind::Whitespace.to_string(), "whitespace");
    assert_eq!(Kind::Controls.to_string(), "controls");
    assert_eq!(Kind::Symbol.to_string(), "symbol");
    assert_eq!(Kind::Special(b':').to_string(), "special");
}
