use crate::Kind::{Controls, Symbol};
use crate::Token;

#[test]
fn properties() {
    let token: Token = Token::new(Symbol, "azAZ09_", 1, 2);
    assert_eq!(token.kind(), Symbol);
    assert_eq!(token.value(), "azAZ09_");
    assert_eq!(token.line(), 1);
    assert_eq!(token.position(), 2);
}

#[test]
fn display() {
    let token: Token = Token::new(Symbol, "azAZ09_", 1, 2);
    let result: String = token.to_string();
    let expected: &str = "Token([1:2]:symbol azAZ09_)";
    assert_eq!(result, expected);

    let token: Token = Token::new(Controls, "\x00\x1F\x7F", 3, 4);
    let result: String = token.to_string();
    let expected: &str = "Token([3:4]:controls ???)";
    assert_eq!(result, expected);
}
