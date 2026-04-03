# lex

[![Crates.io](https://img.shields.io/crates/v/lex.svg)](https://crates.io/crates/lex)
[![Docs.rs](https://docs.rs/lex/badge.svg)](https://docs.rs/lex)
[![License: MIT](https://img.shields.io/crates/l/lex.svg)](https://opensource.org/licenses/MIT)

This library aids in parsing programming languages.

    lex = "0.16.0-rc.2"

There are no dependencies.

## Lexing

The `lexer!` macro defines a token kind enum, implements the `TokenKind` trait, and generates a lexer constructor — all
in one declaration. Rules are matched in declaration order.

```rust,ignore
use lex::lexer::matchers::{digits, ident, whitespace};
use lex::{keyword, lexer, line_comment, literal};

lexer! {
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
    pub enum Kind {
        LineComment: line_comment!("//"),
        Whitespace: whitespace,
        Import: keyword!("import"),
        Ident: ident,
        Integer: digits,
        LBrace: literal!("{"),
        RBrace: literal!("}"),
        Semi: literal!(";"),
    }
}

let tokens = Kind::lexer().lex("import foo;");
```

The `Unrecognized` and `EndOfFile` token variants are added automatically.

## Parsing

The parser provides a token-stream cursor with skip sets, checkpoints for backtracking, multi-error recovery, and
leading comment extraction.

```rust,ignore
use lex::parser::Parser;

let mut parser: Parser<Kind> = Parser::new(source, tokens)
.with_skip(Kind::Whitespace);

let token = parser.expect(Kind::Ident) ?;
```

## Built-in Matchers

- `ident` — `[a-zA-Z_][a-zA-Z0-9_]*`
- `digits` — `[0-9]+`
- `whitespace` — ASCII whitespace
- `literal!("...")` — exact string match
- `keyword!("...")` — exact string match with word boundary
- `line_comment!("//")` — line comment with delimiter
