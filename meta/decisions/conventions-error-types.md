---
id: dec.conventions-error-types
nodes:
  - cairn.root
status: accepted
ratification: local
ratified_by: machine
receipts:
  - rev.conventions-error-types-correctness
  - rev.conventions-error-types-alternatives
date: 2026-08-11
informed_by:
  - res.conventions-error-types-alignment
  - res.authoreval-instrument-evidence
affects:
  - docs/conventions.md
  - tests/conventions.rs
  - meta/decisions/conventions-error-types.md
  - meta/todos/todo.conventions-thiserror-divergence.md
  - meta/research/conventions-error-types-alignment.md
  - meta/todos/todo.public-error-boundary-convention-divergence.md
  - meta/reviews/rev.conventions-error-types-correctness.md
  - meta/reviews/rev.conventions-error-types-alternatives.md
---
# Error types are hand-written without thiserror

## Context

`docs/conventions.md` section "Error Types" required `thiserror::Error` for
all Cairn error types. `res.authoreval-instrument-evidence` section 3 showed
the rule had zero implementations: `thiserror` appears in neither `Cargo.toml`
nor `Cargo.lock`, and `CairnError` (`src/error.rs`) hand-writes its `Display`
implementation. A MUST that nothing satisfies forces every new error type to
choose between the document and the codebase, and each session relitigates
the choice (`todo.conventions-thiserror-divergence`).

## Decision

The convention follows the code. When an error type needs standard-error
formatting, implement `fmt::Display` and `std::error::Error` by hand rather
than through `thiserror::Error`; `src/error.rs` is the reference
implementation. Cairn deliberately carries no `thiserror` dependency.

## Rationale

Adopting `thiserror` instead would require no intended message, code, or variant
change. It would cost a compile-time dependency and a conversion pass over
`CairnError` and the subsystem error enums, purely to make the document true.
Removing the rule has no error-surface change and is the smaller direction.
The blanket form of the old rule was also unsatisfiable as restated:
`SetFieldError` and `BaselineError` are error enums that implement
neither trait, so the surviving convention is conditional on the type needing
standard-error formatting (`res.conventions-error-types-alignment`).

## For

Adopt `thiserror` and retain the old rule. It is a familiar Rust idiom, can
remove hand-written formatting code, and can generate conversions for error
boundaries. The workspace already compiles other procedural macros, so the
marginal toolchain cost is small.

## Against

The rule has zero implementations. Adopting it would add a dependency and
convert stable error code solely to satisfy prose, while a mechanical
conversion could change source chaining. It does not itself resolve the
separate public-boundary divergence, whose correct scope may instead require
narrowing that boundary rule.

## Verdict

The familiar derive is not worth converting stable code without a concrete
error-surface need. The hand-written direction fixes the contradiction with one
documentation change and leaves `thiserror` adoption available through
refinement.

Decision: retain hand-written error formatting.

## Consequences

- `docs/conventions.md` "Error Types" now states the hand-written convention
  and no longer requires `thiserror`.
- `tests/conventions.rs::test_conventions_error_types_match_code` locks both
  sides of the resolved state: the Error Types section must state only the
  hand-written convention, and no workspace package may declare `thiserror`
  under any dependency alias.
- The neighbouring public-boundary rules have a separate proven divergence,
  recorded as `todo.public-error-boundary-convention-divergence`; this ruling
  neither claims nor changes that boundary.
- Adopting `thiserror` later is a refining decision plus the dependency
  change.
- No error message, code, or variant set changed under this ruling.
