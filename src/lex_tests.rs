use crate::{Kind, Lex, Lexer, Token};

#[test]
fn properties() {
    let source: &str = "the source code";
    let tokens: &[Token] = &[
        Token::new(Kind::Symbol, "the", 0, 0),
        Token::new(Kind::Whitespace, " ", 0, 3),
        Token::new(Kind::Symbol, "source", 0, 4),
        Token::new(Kind::Whitespace, " ", 0, 10),
        Token::new(Kind::Symbol, "code", 0, 11),
    ];
    let lex: Lex = Lexer::default().lex(source).unwrap();
    assert_eq!(lex.source(), source);
    assert_eq!(lex.tokens(), tokens);
}

#[test]
fn display() {
    let source: &str = "the code\nsecond line\n";
    let lex: Lex = Lexer::default().lex(source).unwrap();
    let s: String = lex.to_string();
    let result: Vec<&str> = s.split("\n").collect();
    let expected: Vec<&str> = vec![
        "line 1:",
        "    0    : symbol           the",
        "    3    : whitespace       s",
        "    4    : symbol           code",
        "    8    : line-ending      n",
        "line 2:",
        "    0    : symbol           second",
        "    6    : whitespace       s",
        "    7    : symbol           line",
        "    11   : line-ending      n",
        "",
    ];
    assert_eq!(result, expected);
}
