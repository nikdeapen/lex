use lex::lexer::matchers::{digits, ident, whitespace};
use lex::lexer::{Lexer, Token, TokenKind};
use lex::literal;
use lex::parser::Parser;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
enum Kind {
    Whitespace,
    Ident,
    Int,
    LBrace,
    RBrace,
    Eq,
    Semi,
    Unknown,
    Eof,
}

impl TokenKind for Kind {
    fn unknown() -> Self {
        Kind::Unknown
    }

    fn eof() -> Self {
        Kind::Eof
    }

    fn label(&self) -> &'static str {
        match self {
            Kind::Whitespace => "whitespace",
            Kind::Ident => "identifier",
            Kind::Int => "integer",
            Kind::LBrace => "'{'",
            Kind::RBrace => "'}'",
            Kind::Eq => "'='",
            Kind::Semi => "';'",
            Kind::Unknown => "unknown",
            Kind::Eof => "end of file",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Message {
    name: String,
    fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
struct Field {
    type_name: String,
    field_name: String,
    number: String,
}

fn proto_lexer() -> Lexer<Kind> {
    Lexer::default()
        .with_rule(Kind::Whitespace, whitespace)
        .with_rule(Kind::Ident, ident)
        .with_rule(Kind::Int, digits)
        .with_rule(Kind::LBrace, literal!("{"))
        .with_rule(Kind::RBrace, literal!("}"))
        .with_rule(Kind::Eq, literal!("="))
        .with_rule(Kind::Semi, literal!(";"))
}

fn parse_message(p: &mut Parser<Kind>) -> Option<Message> {
    let keyword: Token<Kind> = p.expect(Kind::Ident)?;
    if keyword.text(p.source()) != "message" {
        p.error("expected 'message' keyword");
        return None;
    }

    let name: Token<Kind> = p.expect(Kind::Ident)?;
    let name: String = name.text(p.source()).to_string();

    p.expect(Kind::LBrace)?;

    let mut fields: Vec<Field> = Vec::default();
    while !p.check(Kind::RBrace) && !p.check(Kind::Eof) {
        match parse_field(p) {
            Some(field) => fields.push(field),
            None => {
                p.skip_until(Kind::Semi);
                p.advance(); // consume the ';'
            }
        }
    }

    p.expect(Kind::RBrace)?;

    Some(Message { name, fields })
}

fn parse_field(p: &mut Parser<Kind>) -> Option<Field> {
    let type_token: Token<Kind> = p.expect(Kind::Ident)?;
    let type_name: String = type_token.text(p.source()).to_string();

    let name_token: Token<Kind> = p.expect(Kind::Ident)?;
    let field_name: String = name_token.text(p.source()).to_string();

    p.expect(Kind::Eq)?;

    let number_token: Token<Kind> = p.expect(Kind::Int)?;
    let number: String = number_token.text(p.source()).to_string();

    p.expect(Kind::Semi)?;

    Some(Field {
        type_name,
        field_name,
        number,
    })
}

#[test]
fn fn_parse_message() {
    let source: String = "message Foo { string name = 1; int32 id = 2; }".to_string();
    let lexer: Lexer<Kind> = proto_lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(source, tokens).with_skip(Kind::Whitespace);

    let message: Option<Message> = parse_message(&mut parser);

    assert!(parser.errors().is_empty(), "errors: {:?}", parser.errors());
    assert_eq!(
        message,
        Some(Message {
            name: "Foo".to_string(),
            fields: vec![
                Field {
                    type_name: "string".to_string(),
                    field_name: "name".to_string(),
                    number: "1".to_string(),
                },
                Field {
                    type_name: "int32".to_string(),
                    field_name: "id".to_string(),
                    number: "2".to_string(),
                },
            ],
        })
    );
}

#[test]
fn fn_parse_message_empty() {
    let source: String = "message Empty {}".to_string();
    let lexer: Lexer<Kind> = proto_lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(source, tokens).with_skip(Kind::Whitespace);

    let message: Option<Message> = parse_message(&mut parser);

    assert!(parser.errors().is_empty());
    assert_eq!(
        message,
        Some(Message {
            name: "Empty".to_string(),
            fields: vec![],
        })
    );
}

#[test]
fn fn_parse_message_missing_brace() {
    let source: String = "message Foo string name = 1; }".to_string();
    let lexer: Lexer<Kind> = proto_lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(source, tokens).with_skip(Kind::Whitespace);

    let message: Option<Message> = parse_message(&mut parser);

    assert!(message.is_none());
    assert_eq!(parser.errors().len(), 1);
    assert_eq!(
        parser.errors()[0].message(),
        "expected '{', found identifier"
    );
}

#[test]
fn fn_parse_message_recovery() {
    let source: String = "message Foo { string = 1; int32 id = 2; }".to_string();
    let lexer: Lexer<Kind> = proto_lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(source, tokens).with_skip(Kind::Whitespace);

    let message: Option<Message> = parse_message(&mut parser);

    // First field fails (missing field name), recovery skips to ';'
    // Second field parses successfully
    assert_eq!(parser.errors().len(), 1);
    assert_eq!(
        message,
        Some(Message {
            name: "Foo".to_string(),
            fields: vec![Field {
                type_name: "int32".to_string(),
                field_name: "id".to_string(),
                number: "2".to_string(),
            }],
        })
    );
}

#[test]
fn fn_parse_message_multiline() {
    let source: String = r#"message Person {
    string name = 1;
    string email = 2;
    int32 age = 3;
}"#
    .to_string();
    let lexer: Lexer<Kind> = proto_lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(source, tokens).with_skip(Kind::Whitespace);

    let message: Option<Message> = parse_message(&mut parser);

    assert!(parser.errors().is_empty(), "errors: {:?}", parser.errors());
    let message: Message = message.unwrap();
    assert_eq!(message.name, "Person");
    assert_eq!(message.fields.len(), 3);
    assert_eq!(message.fields[0].field_name, "name");
    assert_eq!(message.fields[1].field_name, "email");
    assert_eq!(message.fields[2].field_name, "age");
}
