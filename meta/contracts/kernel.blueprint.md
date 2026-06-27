---
node: cairn.kernel.blueprint
---

# Contract: cairn.kernel.blueprint

## Purpose

The blueprint front end: it reads a `.blueprint` file and turns its text into a
typed, source-positioned AST. A hand-written lexer tokenizes the source, a
recursive descent parser builds the node and edge tree, and every diagnostic
carries a stable error code and a one-based source span. This is the entry point
all higher kernel modules use to understand a Cairn project.

## Public interface

- `parse_file(path)`: reads a `.blueprint` file from disk and returns an `Ast`,
  or a `ParseError` on I/O failure or malformed source.
- `Ast`: parsed root holding `nodes: Vec<Node>` and `edges: Vec<Edge>`.
- `Node`: declaration with `kind`, `name`, `description`, `id`, `tags`, `paths`,
  `owns_files`, `contracts`, `raw_fields`, nested `children`, and `span`.
- `NodeKind`: enum over `System`, `Container`, `Module`, `Actor`.
- `Edge`: dependency edge with `from`, `to`, `description`, and `span`.
- `Field`: retained non-contract field metadata (`name`, `values`, `span`).
- `Span`: one-based `file`, `line`, `column`, `end_line`, `end_column`.
- `ParseError` / `ParseErrorKind`: error type with a stable `code` (for example
  `CAIRN_PARSE_UNEXPECTED_TOKEN`), `message`, `span`, and `kind`
  (`UnexpectedToken`, `UnterminatedString`, `MissingField`).

## Invariants

- Every produced `Span` uses one-based line and column positions.
- Every `ParseError` carries a stable, non-empty `code` and a source `span`.
- An unterminated string literal yields `ParseErrorKind::UnterminatedString`.
- A node missing a required field yields `ParseErrorKind::MissingField`.
- A word is `is_ascii_alphanumeric` or one of `_`, `-`, `.`.
- `parse_file` and `parse_str` are pure with respect to the AST: identical source
  yields an identical `Ast`.

## Dependencies

Leaf module with no outgoing blueprint edges. It is consumed by
`cairn.kernel.map`, `cairn.kernel.scanner`, and `cairn.kernel.changes`, which all
take the parsed `Ast` as their input. Internally it shells out only to the
standard library (`std::fs`, `std::path`).

## Tests

A `#[cfg(test)] mod tests` lives at the foot of `src/blueprint/lexer.rs` covering
tokenization (words, strings, tags, punctuation, arrows) and the unterminated
string error path. Parser behavior is exercised through the consuming kernel
modules and the integration fixtures.
