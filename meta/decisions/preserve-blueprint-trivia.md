---
id: dec.preserve-blueprint-trivia
nodes:
  - cairn.kernel.changes
status: accepted
date: 2026-06-23
---

# Archive preserves blueprint comments via a source-preserving delta splice

## Context

`apply_blueprint_delta` parsed `cairn.blueprint` into an AST, mutated it, and
re-emitted the whole tree with `serialize_ast`. That serializer reconstructs the
file from typed nodes alone, so every comment and blank line was discarded on
archive. PR #157 (bead `cairn-giv`) only fixed the empty-delta case by skipping
the rewrite entirely. For a non-empty delta (a real node or edge change) the
rewrite still ran and still stripped trivia (bead `cairn-2sh`).

The prior session deferred this pending a deliberate trivia-model decision. Two
designs were on the table:

1. **Full trivia model.** Capture comment and blank-line trivia in the lexer,
   attach it to every AST node/field/edge, and re-emit it from the serializer.
   Full round-trip fidelity, but it touches the whole parse/serialize pipeline,
   adds trivia fields to `Node`/`Edge` (which derive `Eq`/`Hash`), and changes
   the AST `Hash` that `scanner/cache.rs` uses as the reconciler cache key.
2. **Source-preserving splice.** Keep the original source text and rewrite only
   the declarations the delta actually changes.

The maintainer chose option 2: the conservative, contained option that fully
solves the stated problem (comments survive a structural-delta archive).

## Decision

`apply_blueprint_delta` delegates to `src/changes/apply/preserve.rs`, which
applies the delta against the original source string:

- Untouched lines (comments, blank lines, unchanged declarations) are copied
  through byte-for-byte, at every nesting depth.
- A node whose subtree is unchanged is copied verbatim; a node that is itself a
  `modified` target is re-serialised wholesale; a node that is only renamed or
  only has a changed descendant is recursed into, preserving its own trivia.
- Top-level edges are kept verbatim unless an endpoint rename, removal, or
  modification changes them; added/replaced edges and added nodes are appended.

Node line extents come from the lexer token stream, not the AST: the k-th `{`
token opens the k-th node in preorder, so a brace stack pairs each opener with
its closer. This needs no change to the AST shape, the `Span` semantics, the
`Node`/`Edge` `Eq`/`Hash` derives, the reconciler cache key, or the `query_api`
span JSON. `serialize_ast` (the whole-tree serializer) is removed; `serialize_node`
remains for the canonical forms of changed and added nodes.

## Rationale

The token-stream extent derivation is string-safe (a `{` inside a quoted
description is a `String` token, never an `OpenBrace`), so brace matching is
robust. Confining the work to `src/changes/apply` keeps the blast radius to one
module: no kernel parser, AST, or cache surface moves, so the change cannot
regress reconciliation or the cache contract.

## Consequences

- A node that is a `modified` target is reproduced from the delta in canonical
  form, so comments and blank lines *inside that one declaration* are not
  retained. This is inherent: the declaration's body is replaced wholesale.
- A removed node's own trivia is removed with it; a comment that sat above the
  removed node is left in place and may read as orphaned.
- Added nodes and added/replaced edges are appended in canonical serialised form
  at end of file. The blueprint parser is order-independent (nodes and edges may
  interleave), so this is purely cosmetic for the rare add-on-archive case.
- Full round-trip fidelity (preserving comments inside a modified declaration)
  is explicitly deferred. Reaching it requires the option-1 trivia model and a
  reconciler-cache version bump; revisit only if a real need appears.
