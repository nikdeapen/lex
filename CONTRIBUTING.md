# Contributing

This guide outlines how contributions should be made to the codebase.

## Pull Requests

The `master` branch is protected, so every change is made on a branch & merged with a pull request.

- Each pull request is one small, focused change.
- Pull requests are squash merged, so the title becomes the commit message.
- The build workflow must pass. It runs the same checks as:

```sh
cargo fmt --check
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Commits

Each commit is one small, focused change. Its message is a tag & one sentence:

```text
[parser] Added support for checkpointing.
```

The description should be a bulleted list of changes. The list of changes per PR should be 
cohesive. If there are many unrelated changes they should be in separate PRs.

### Tags

- `[cargo]`: the project, typically anything focused outside of `src/` & `tests/`, like the
  manifest, the readme, the build workflow & version updates.
- `[lexer]`: the lexer in `src/lexer/`.
- `[parser]`: the parser in `src/parser/` & its end-to-end tests in `tests/`.
- `[*]`: a change that spans the lexer & the parser.

## Versions

- Breaking changes bump the minor version while the crate is below `1.0`.
