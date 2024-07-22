use crate::Token;

/// Parses block comments.
pub fn block_comments<F>(token: Token, delimiter: &str, mut add_fn: F)
where
    F: FnMut(&str),
{
    let (mut lines, last) = split_last_line(token.value());
    let indent_level: usize = get_indent_level(last);
    while !lines.is_empty() {
        lines = strip_line_ending(lines);
        let (not_last, last) = split_last_line(lines);
        lines = not_last;

        let last_indent_level: usize = get_indent_level(last);
        if indent_level != last_indent_level {
            return;
        }

        let last: &str = last.trim();
        if !last.starts_with(delimiter) {
            return;
        }
        let last: &str = &last[delimiter.len()..];

        add_fn(last.trim())
    }
}

/// Splits the last line from the string.
///
/// Returns `(not_last_line, last_line)`.
fn split_last_line(s: &str) -> (&str, &str) {
    if let Some(rn) = s
        .as_bytes()
        .iter()
        .rposition(|c| *c == b'\r' || *c == b'\n')
    {
        s.split_at(rn + 1)
    } else {
        ("", s)
    }
}

/// Strips the line-ending from the string.
///
/// The string must end with a line-ending.
fn strip_line_ending(s: &str) -> &str {
    debug_assert!(s.ends_with("\r") || s.ends_with("\n") || s.ends_with("\r\n"));

    if s.ends_with("\r") {
        &s[..s.len() - 1]
    } else if s.ends_with("\n") {
        &s[..s.len() - 1]
    } else if s.ends_with("\r\n") {
        &s[..s.len() - 2]
    } else {
        unreachable!()
    }
}

/// Gets the indent level of the string. (one indent is 1 tab or 4 spaces)
fn get_indent_level(s: &str) -> usize {
    let mut s: &str = s;
    let mut indent_level: usize = 0;
    loop {
        if s.starts_with("\t") {
            indent_level += 1;
            s = &s[1..];
        } else if s.starts_with("    ") {
            indent_level += 1;
            s = &s[4..];
        } else {
            return indent_level;
        }
    }
}
