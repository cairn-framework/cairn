---
id: res.ui-query-api-wire-fork
nodes: [cairn.ui]
date: 2026-07-08
method: primary
---

# UI query_api wire-shape fork: investigation

## Question
Should `src/ui` (webui HTTP server + `app.js`) consume `query_api`'s canonical
wire shapes (Strategy B) or keep its own wire and have the server translate
(Strategy A)?

## Evidence (explore agent, 2026-07-08)
- `app.js` is the only browser consumer of `/api/*` (~15 parse sites; lowercase
  enums, raw artefacts, full records). `src/ui/mod.rs` is a second consumer.
  No LSP/export consumer hits `/api/`.
- `src/ui/api.rs` + `src/ui/serialise.rs` rebuild ~10 JSON shapes that already
  exist as `src/query_api` handlers; the two differ for all 15 endpoints
  (lowercase vs `Debug` enums, raw vs structured artefacts, full records vs ID
  lists in `depends`).
- `tests/wire_format_snapshots.rs` is byte-identical for 14 of 15 endpoints
  (all except `beads`/`symbols`).
- `query_api` has no `format` negotiation field; its serialiser emits `Debug`
  enum cases (`serialise.rs:10,19`).
- Three endpoints have no `query_api` handler: `/api/meta`, `/api/blueprint`,
  `/api/node/{n}/beads` — genuine gaps under either strategy.

## Adversarial debate (2026-07-08)
- ReformerB (Strategy B): the accepted invariant (dec.no-orchestrator,
  todo.simplify-architecture) is "one canonical JSON shape in query_api; every
  surface a thin consumer". Only B achieves this and satisfies the literal
  acceptance ("no JSON shape-building in src/ui"). A keeps a second builder;
  hybrid puts a second serialiser in the spine.
- ConservativeA (Strategy A): the goal is "one source of truth for DATA", not
  "one wire format"; the UI wire is a legitimate presentation boundary; B's
  app.js rewrite + 14 snapshot churn is disproportionate and risky; snapshots
  should stay byte-identical to avoid silent UI regressions; A routes all
  computation through query_api handlers (eliminating the real duplicated
  computation) while keeping a thin mapping.

## Outcome
Maintainer ratified **Strategy B** (2026-07-08): adopt query_api wire, delete
`src/ui/api.rs` + `src/ui/serialise.rs`, rewrite `app.js` + `src/ui/mod.rs`,
rebase 14 snapshots as recorded schema decisions. Implementation deferred to a
later dev-loop session (session signing off).
