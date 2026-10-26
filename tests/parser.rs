use lex::lexer::matchers::{digits, ident, whitespace};
use lex::lexer::{Lexer, Span, Token};
use lex::parser::{ParseError, Parser};
use lex::{lexer, line_comment, literal};

lexer! {
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    enum Kind {
        Whitespace: whitespace,
        LineComment: line_comment!("//"),
        Ident: ident,
        Int: digits,
        LBrace: literal!("{"),
        RBrace: literal!("}"),
        Eq: literal!("="),
        Semi: literal!(";"),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Error {
    Unexpected(ParseError<Kind>),
    ExpectedMessageKeyword(Span),
}

impl From<ParseError<Kind>> for Error {
    fn from(error: ParseError<Kind>) -> Self {
        Self::Unexpected(error)
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

fn parse_message(p: &mut Parser<'_, Kind>, errors: &mut Vec<Error>) -> Result<Message, Error> {
    if p.accept_text(Kind::Ident, "message").is_none() {
        return Err(Error::ExpectedMessageKeyword(p.peek().span()));
    }

    let name: Token<Kind> = p.expect(Kind::Ident)?;
    let name: String = p.text(name.span()).to_string();

    p.expect(Kind::LBrace)?;

    let mut fields: Vec<Field> = Vec::default();
    while !p.check(Kind::RBrace) && !p.check(Kind::EndOfFile) {
        match parse_field(p) {
            Ok(field) => fields.push(field),
            Err(error) => {
                errors.push(error);
                p.skip_until(Kind::Semi);
                p.advance(); // consume the ';'
            }
        }
    }

    p.expect(Kind::RBrace)?;

    Ok(Message { name, fields })
}

fn parse_field(p: &mut Parser<'_, Kind>) -> Result<Field, Error> {
    let type_token: Token<Kind> = p.expect(Kind::Ident)?;
    let type_name: String = p.text(type_token.span()).to_string();

    let name_token: Token<Kind> = p.expect(Kind::Ident)?;
    let field_name: String = p.text(name_token.span()).to_string();

    p.expect(Kind::Eq)?;

    let number_token: Token<Kind> = p.expect(Kind::Int)?;
    let number: String = p.text(number_token.span()).to_string();

    p.expect(Kind::Semi)?;

    Ok(Field {
        type_name,
        field_name,
        number,
    })
}

#[test]
fn fn_parse_message() {
    let source: String = "message Foo { string name = 1; int32 id = 2; }".to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    assert!(errors.is_empty(), "errors: {:?}", errors);
    assert_eq!(
        message,
        Ok(Message {
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
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    assert!(errors.is_empty());
    assert_eq!(
        message,
        Ok(Message {
            name: "Empty".to_string(),
            fields: vec![],
        })
    );
}

#[test]
fn fn_parse_message_missing_brace() {
    let source: String = "message Foo string name = 1; }".to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    let Err(Error::Unexpected(error)) = message else {
        panic!("expected an unexpected token error: {:?}", message);
    };
    assert_eq!(error.expected(), Kind::LBrace);
    assert_eq!(error.found().kind(), Kind::Ident);
    assert_eq!(error.to_string(), "expected LBrace, found Ident");
    assert!(errors.is_empty());
}

#[test]
fn fn_parse_message_contextual_keyword() {
    let source: String = "message message { message message = 1; }".to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    assert!(errors.is_empty(), "errors: {:?}", errors);
    assert_eq!(
        message,
        Ok(Message {
            name: "message".to_string(),
            fields: vec![Field {
                type_name: "message".to_string(),
                field_name: "message".to_string(),
                number: "1".to_string(),
            }],
        })
    );
}

#[test]
fn fn_parse_message_missing_keyword() {
    let source: String = "struct Foo {}".to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    assert_eq!(message, Err(Error::ExpectedMessageKeyword(Span::new(0, 6))));
}

#[test]
fn fn_parse_message_recovery() {
    let source: String = "message Foo { string = 1; int32 id = 2; }".to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    // First field fails (missing field name), recovery skips to ';'
    // Second field parses successfully
    assert_eq!(errors.len(), 1);
    assert_eq!(
        message,
        Ok(Message {
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
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens).with_skip(Kind::Whitespace);

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<Message, Error> = parse_message(&mut parser, &mut errors);

    assert!(errors.is_empty(), "errors: {:?}", errors);
    let message: Message = message.unwrap();
    assert_eq!(message.name, "Person");
    assert_eq!(message.fields.len(), 3);
    assert_eq!(message.fields[0].field_name, "name");
    assert_eq!(message.fields[1].field_name, "email");
    assert_eq!(message.fields[2].field_name, "age");
}

#[derive(Debug, Eq, PartialEq)]
struct CommentedMessage {
    comments: Vec<String>,
    name: String,
    fields: Vec<CommentedField>,
}

#[derive(Debug, Eq, PartialEq)]
struct CommentedField {
    comments: Vec<String>,
    type_name: String,
    field_name: String,
    number: String,
}

fn parse_commented_message(
    p: &mut Parser<'_, Kind>,
    errors: &mut Vec<Error>,
) -> Result<CommentedMessage, Error> {
    let comments: Vec<String> = p
        .leading_comments()
        .iter()
        .map(|s| s.text(p.source()).to_string())
        .collect();

    if p.accept_text(Kind::Ident, "message").is_none() {
        return Err(Error::ExpectedMessageKeyword(p.peek().span()));
    }

    let name: Token<Kind> = p.expect(Kind::Ident)?;
    let name: String = p.text(name.span()).to_string();

    p.expect(Kind::LBrace)?;

    let mut fields: Vec<CommentedField> = Vec::default();
    while !p.check(Kind::RBrace) && !p.check(Kind::EndOfFile) {
        match parse_commented_field(p) {
            Ok(field) => fields.push(field),
            Err(error) => {
                errors.push(error);
                p.skip_until(Kind::Semi);
                p.advance();
            }
        }
    }

    p.expect(Kind::RBrace)?;

    Ok(CommentedMessage {
        comments,
        name,
        fields,
    })
}

fn parse_commented_field(p: &mut Parser<'_, Kind>) -> Result<CommentedField, Error> {
    let comments: Vec<String> = p
        .leading_comments()
        .iter()
        .map(|s| s.text(p.source()).to_string())
        .collect();

    let type_token: Token<Kind> = p.expect(Kind::Ident)?;
    let type_name: String = p.text(type_token.span()).to_string();

    let name_token: Token<Kind> = p.expect(Kind::Ident)?;
    let field_name: String = p.text(name_token.span()).to_string();

    p.expect(Kind::Eq)?;

    let number_token: Token<Kind> = p.expect(Kind::Int)?;
    let number: String = p.text(number_token.span()).to_string();

    p.expect(Kind::Semi)?;

    Ok(CommentedField {
        comments,
        type_name,
        field_name,
        number,
    })
}

#[test]
fn fn_parse_message_comments() {
    let source: String = r#"// Describes the message.
// Second line.
message Person {
    // The person's name.
    string name = 1;
    // The person's age.
    int32 age = 2;
}"#
    .to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens)
        .with_skip(Kind::Whitespace)
        .with_skip(Kind::LineComment)
        .with_line_comment(Kind::LineComment, "//");

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<CommentedMessage, Error> =
        parse_commented_message(&mut parser, &mut errors);

    assert!(errors.is_empty(), "errors: {:?}", errors);
    let message: CommentedMessage = message.unwrap();

    assert_eq!(
        message.comments,
        vec![" Describes the message.", " Second line.",]
    );
    assert_eq!(message.name, "Person");
    assert_eq!(message.fields.len(), 2);

    assert_eq!(message.fields[0].comments, vec![" The person's name."]);
    assert_eq!(message.fields[0].field_name, "name");

    assert_eq!(message.fields[1].comments, vec![" The person's age."]);
    assert_eq!(message.fields[1].field_name, "age");
}

#[test]
fn fn_parse_message_no_comments() {
    let source: String = "message Foo { string name = 1; }".to_string();
    let lexer: Lexer<Kind> = Kind::lexer();
    let tokens: Vec<Token<Kind>> = lexer.lex(&source);
    let mut parser: Parser<Kind> = Parser::new(&source, tokens)
        .with_skip(Kind::Whitespace)
        .with_skip(Kind::LineComment)
        .with_line_comment(Kind::LineComment, "//");

    let mut errors: Vec<Error> = Vec::default();
    let message: Result<CommentedMessage, Error> =
        parse_commented_message(&mut parser, &mut errors);

    assert!(errors.is_empty(), "errors: {:?}", errors);
    let message: CommentedMessage = message.unwrap();
    assert!(message.comments.is_empty());
    assert!(message.fields[0].comments.is_empty());
}
