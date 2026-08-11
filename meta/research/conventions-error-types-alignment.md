---
id: res.conventions-error-types-alignment
nodes:
  - cairn.root
date: 2026-08-11
method: primary
---

# Error-type convention aligned to the code, not the other way round

`todo.conventions-thiserror-divergence` resolved the divergence that
`res.authoreval-instrument-evidence` section 3 reported: `docs/conventions.md`
required `thiserror::Error` for all error types while the crate declared no
such dependency and `src/error.rs` hand-writes `Display`.

## Direction taken

The rule was removed, not the dependency adopted. `CairnError` and its seven
variants already work; preserving their current messages, codes, and variant
set requires no `thiserror` dependency. Adopting it would add compile-time
cost and could change source chaining if conversion were mechanical. The
section now reads:
implement `fmt::Display` and `std::error::Error` by hand when standard-error
formatting is needed, with `src/error.rs` as the reference implementation.

`tests/conventions.rs::test_conventions_error_types_match_code` locks both
sides of the resolved state: the Error Types section must state only the
hand-written convention, and no workspace package may declare `thiserror`
under any dependency alias.

## What the audit showed beyond the todo

1. A blanket "all error types implement `Display` and `std::error::Error`"
   would also have been false: `SetFieldError`
   (`src/artefacts/frontmatter.rs`) and `BaselineError`
   (`src/summariser/baseline.rs`) are error enums implementing neither trait.
   The final wording is therefore conditional ("when an error type needs
   standard-error formatting") rather than universal.
2. The neighbouring public-boundary bullets are proven false as written:
   public `artefacts::frontmatter` functions return `SetFieldError`, public UI
   APIs return `UiError`, and neither converts to `CairnError`. This is outside
   the formatting-rule unit and is now tracked by
   `todo.public-error-boundary-convention-divergence`.

## Limits

This is a documentation alignment; no error behaviour, message text, code, or
variant set changed. The audit covered `src/` only.
