# lex

[![Crates.io](https://img.shields.io/crates/v/lex.svg)](https://crates.io/crates/lex)
[![Docs.rs](https://docs.rs/lex/badge.svg)](https://docs.rs/lex)
[![License: MIT](https://img.shields.io/crates/l/lex.svg)](https://opensource.org/licenses/MIT)

This library aids in parsing programming languages.

    lex = "0.0.0"

There are no dependencies.

## Lexer

The lexer converts source text into a sequence of tokens using ordered rules. Tokens are generic
over a user-defined `TokenKind` enum. The lexer is error-free — all input is tokenizable.

```rust
use lex::lexer::matchers::{ident, whitespace};
use lex::lexer::{Lexer, Token, TokenKind};
use lex::literal;

let lexer: Lexer<Kind> = Lexer::default ()
.with_rule(Kind::Whitespace, whitespace)
.with_rule(Kind::Ident, ident)
.with_rule(Kind::Semi, literal!(";"));

let tokens: Vec<Token<Kind> > = lexer.lex("let x;");
```

## Parser

The parser provides a token-stream cursor with skip sets, checkpoints for backtracking,
multi-error recovery, and leading comment extraction.

```rust
use lex::parser::Parser;

let mut parser: Parser<Kind> = Parser::new(source, tokens)
.with_skip(Kind::Whitespace);

let token: Token<Kind> = parser.expect(Kind::Ident) ?;
```

## Built-in Matchers

- `ident` — `[a-zA-Z_][a-zA-Z0-9_]*`
- `digits` — `[0-9]+`
- `whitespace` — ASCII whitespace
- `literal!("...")` — exact string match
- `line_comment!("//")` — line comment with delimiter
