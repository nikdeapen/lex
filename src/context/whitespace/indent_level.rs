use crate::ParseContext;

impl<'a> ParseContext<'a> {
    // Indent Level

    /// Gets the indent level and the indent length.
    pub(crate) fn indent_level_and_len(&self) -> (usize, usize) {
        let mut indent_level: usize = 0;
        let mut len: usize = 0;

        let spaces_per_tab: usize = self.config().spaces_per_tab();
        let (white, _after_white) = self.whitespace();
        if let Some(white) = white {
            let mut white: &str = white.value();
            while !white.is_empty() {
                if white.as_bytes()[0] == b'\t' {
                    indent_level += 1;
                    len += 1;
                    white = &white[1..];
                } else if white.as_bytes()[0] == b' ' {
                    if white.len() >= spaces_per_tab
                        && white.as_bytes()[..spaces_per_tab]
                            .iter()
                            .all(|c| *c == b' ')
                    {
                        indent_level += 1;
                        len += spaces_per_tab;
                        white = &white[spaces_per_tab..];
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        (indent_level, len)
    }
}
