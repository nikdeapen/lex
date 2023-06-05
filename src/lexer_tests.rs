use crate::{Kind, Lex, Lexer, Token};

#[test]
fn lex() {
    let test_cases: &[(&str, &[Token])] = &[
        ("", &[]),
        ("你好", &[Token::new(Kind::NonAscii, "你好", 0, 0)]),
        (
            "\x7F你好\x7F",
            &[
                Token::new(Kind::Controls, "\x7F", 0, 0),
                Token::new(Kind::NonAscii, "你好", 0, 1),
                Token::new(Kind::Controls, "\x7F", 0, 7),
            ],
        ),
        (
            "\r \r\r \r\n \n\r",
            &[
                Token::new(Kind::LineEnding, "\r", 0, 0),
                Token::new(Kind::Whitespace, " ", 1, 0),
                Token::new(Kind::LineEnding, "\r", 1, 1),
                Token::new(Kind::LineEnding, "\r", 2, 0),
                Token::new(Kind::Whitespace, " ", 3, 0),
                Token::new(Kind::LineEnding, "\r\n", 3, 1),
                Token::new(Kind::Whitespace, " ", 4, 0),
                Token::new(Kind::LineEnding, "\n", 4, 1),
                Token::new(Kind::LineEnding, "\r", 5, 0),
            ],
        ),
        (
            " \tx\t ",
            &[
                Token::new(Kind::Whitespace, " \t", 0, 0),
                Token::new(Kind::Symbol, "x", 0, 2),
                Token::new(Kind::Whitespace, "\t ", 0, 3),
            ],
        ),
        (
            "\x00\r\x1F\n\x7F\t",
            &[
                Token::new(Kind::Controls, "\x00", 0, 0),
                Token::new(Kind::LineEnding, "\r", 0, 1),
                Token::new(Kind::Controls, "\x1F", 1, 0),
                Token::new(Kind::LineEnding, "\n", 1, 1),
                Token::new(Kind::Controls, "\x7F", 2, 0),
                Token::new(Kind::Whitespace, "\t", 2, 1),
            ],
        ),
        (
            "azAZ09_!",
            &[
                Token::new(Kind::Symbol, "azAZ09_", 0, 0),
                Token::new(Kind::Special(b'!'), "!", 0, 7),
            ],
        ),
    ];
    for (source, expected) in test_cases {
        let result: Lex = Lexer::default().lex(&source).unwrap();
        assert_eq!(result.tokens(), *expected);
    }
}
