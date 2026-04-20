# Docs Staleness Audit
**Status:** done
**Last updated:** 2026-04-16
**Updated by:** Uruk-hai scout

## Summary

3 files in `docs/`. All are at least partially outdated relative to the current phase spec structure in `openspec/specs/` and the campaign plan at `meta/campaigns/rust-full-spec.md`. One file (`docs/spec.md`) is now superseded as an implementation reference by the dedicated OpenSpec capability specs. One file (`docs/blueprint.md`) is partially stale. One file (`docs/phase-2-deferrals.md`) is almost entirely stale and contradicts the current phase plan.

## Findings

### `docs/spec.md` — PARTIALLY_STALE (still authoritative; superseded as implementation reference)

- This is Cairn v0.6, the canonical product specification.
- `meta/campaigns/rust-full-spec.md` line 297 explicitly cites `docs/spec.md` as the authoritative reference. It is **not superseded** as a product truth document.
- However, it is no longer the implementation reference for individual capabilities. The OpenSpec phase specs in `openspec/specs/` (parser/spec.md, query/spec.md, cli/spec.md) are the authoritative implementation contracts for Phase 1 work, derived from spec.md sections 6/7/10/12.
- One concrete stale element: section 17 says "A specific implementation language for the toolchain" is not committed to. The Rust rewrite decision (campaign, phase-0-foundation) has since resolved this — Rust is the target language. The spec body does not reflect this.
- Section 14 phase map does not include Phase 0 (Rust foundation) or Phase 2.5 (graph explorer), both of which exist in the campaign plan (`meta/campaigns/rust-full-spec.md` lines 66-78).
- Overall: still essential reading; contains the product definition no other file duplicates. But treat implementation phasing in section 14 as superseded by `meta/campaigns/rust-full-spec.md`.

### `docs/blueprint.md` — PARTIALLY_STALE (superseded in substance by `openspec/specs/parser/spec.md`)

- blueprint grammar overview is consistent with `docs/spec.md` section 7 and `openspec/specs/parser/spec.md`.
- Line 40: "only leaves claim paths for duplicate checking in this MVP" — uses the word "MVP", which is banned per campaign language rules (`meta/campaigns/rust-full-spec.md` line 286). Minor but inconsistent.
- Line 40: "Leaf paths must be unique. Internal container paths are allowed, but only leaves claim paths for duplicate checking in this MVP." — This is weaker than spec.md section 7, which states path uniqueness applies across all nodes including multi-path lists, with ties illegal at parse time.
- The artefact pointer key list (line 46-52: contract, todos, decisions, research, reviews, sources) matches spec.md section 8.
- No mention of multi-path syntax (`path ["./a", "./b"]`), which spec.md section 7 requires the parser to handle — and `openspec/specs/parser/spec.md` scenario "Path list is normalized" tests for. `docs/blueprint.md` predates this addition.
- Verdict: the grammar overview is mostly accurate but missing multi-path syntax coverage and uses "MVP" language. Needs a one-pass update.

### `docs/phase-2-deferrals.md` — STALE (contradicts current phase plan; superseded by campaign spec)

- This document describes a "kernel MVP" that deliberately defers: scanner/filesystem walking, contract artefact parsing, change directories, archive command, rename propagation, hooks, edge validation, docstring generation, MCP, LSP, summariser, brownfield extraction.
- Every single item in this deferral list is now an explicitly scoped phase in `meta/campaigns/rust-full-spec.md`:
  - Scanner/filesystem walking → Phase 1 (`phase-1-kernel`)
  - Contract artefact parsing → Phase 1 (reconciler, basic scanner)
  - Change directories, archive, rename → Phase 3 (`phase-3-changes`)
  - Hooks → Phase 4 (`phase-4-hooks`)
  - Edge validation, docstring generation → Phase 5 (`phase-5-edges-docstrings`)
  - MCP → Phase 7 (`phase-7-mcp`)
  - Summariser → Phase 8 (`phase-8-summariser`)
  - Brownfield extraction → Phase 9 (`phase-9-brownfield`)
- The framing "these are separate changes after the query layer proves useful" reflects pre-Rust-rewrite thinking. The Rust full-product campaign does not treat these as deferred; they are scheduled phases with full specs.
- Line 3: uses the concept of a "kernel MVP" — the exact framing the campaign explicitly bans (language rules, `meta/campaigns/rust-full-spec.md` line 286).
- This file should either be deleted or archived. It adds confusion to any agent reading `docs/` as context.

## OpenSpec Specs Inventory

| File | Phase | Status |
|------|-------|--------|
| `openspec/specs/cli/spec.md` | Phase 1c | Active — CLI capability spec for Rust implementation |
| `openspec/specs/parser/spec.md` | Phase 1a | Active — Parser capability spec for Rust implementation |
| `openspec/specs/query/spec.md` | Phase 1 | Active — Query semantics spec, protocol-neutral |

No spec files exist yet for Phases 2–10. These are expected: per the campaign, spec writing is a sequential process.

## Recommendations

1. `docs/phase-2-deferrals.md` — delete or move to `openspec/changes/archive/`. It is actively misleading given the current campaign.
2. `docs/blueprint.md` — add multi-path syntax example; replace "MVP" with "initial implementation" or similar; note it is a grammar overview supplement, not the authoritative parser spec.
3. `docs/spec.md` — consider adding a one-line note at the top of section 14 (phase map) and section 17 (non-commitments) pointing to `meta/campaigns/rust-full-spec.md` for current phasing and language decisions.
