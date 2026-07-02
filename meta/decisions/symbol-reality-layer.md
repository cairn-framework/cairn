---
id: dec.symbol-reality-layer
nodes:
  - cairn.kernel
  - cairn.reconcile
status: accepted
date: 2026-07-02
informed_by: [res.vision-refactor-audit]
revisit_triggers:
  - "A fifth reconciler language is added and the SymbolKind enum needs a member no existing mapping covers"
  - "cairn symbols response payload becomes a measured performance problem on large repos"
---

# Symbol reality layer: structured extraction survives past the fingerprint

## Context

`res.vision-refactor-audit` finding 1: every reconciler extracts an identifier,
its kind, and its source location while walking Tree-sitter nodes, then
immediately flattens all three into a single normalised string fed only to the
interface fingerprint. Nothing structured survives. `cairn files <node>`
answers "which files does this module claim", never "what does this module
expose". Agents and orchestrators walking the graph have no queryable
below-module reality layer.

## Decision

Introduce `SymbolRecord { name, kind, signature, file, line, end_line }` and a
language-agnostic `SymbolKind` enum in `src/reconcile/symbol.rs`. All four
reconcilers (Rust, TypeScript, Python, Go) build a `SymbolRecord` alongside the
existing flattened signature string at the same walk point, without changing
how the flattened string is computed. Records thread through
`ReconcileReport`/`TargetReport`, attach to `NodeRecord.symbols` in the graph,
and are queryable via a new `cairn symbols <node>` tool (CLI + MCP) and shown
in the webui's module inspector.

The blueprint stays module-level: this is reality-layer detail, not a new
blueprint concept. `dec.domain-expandability`'s module-level blueprint
boundary is unaffected.

## Rationale

The fingerprint hash (`InterfaceFingerprint`, `src/reconcile/fingerprint.rs`)
must stay byte-identical: the `signature` field is fed by the exact same string
that already feeds the hash, one variable, two consumers. This preserves
`dec.graph-root-fingerprint`'s existing per-target hash gate untouched while
adding a structured view for consumers that need name/kind/location rather
than an opaque hash.

## Consequences

- Cache schema version bumps (`RECONCILER_CACHE_VERSION` 4→5); existing caches
  recompute rather than silently serving stale structure-free data.
- `cairn symbols` becomes the below-module query surface `dec.spec` §5's
  non-goal ("function-level mapping in the blueprint") already anticipated:
  detail is a zoom parameter of queries, never of the blueprint (spec v0.8 §5).
- Enables `dec.persistent-map-snapshot` (symbols persist in `map.json`) and
  `dec.generative-bundles-and-gaps` (dependency interfaces in a bundle come
  from `NodeRecord.symbols`).
