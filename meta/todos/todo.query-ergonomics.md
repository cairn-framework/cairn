---
node: cairn.kernel.query
status: done
created: 2026-07-12
---

# Query Ergonomics

gh:#239

Remaining node-query ergonomics gaps in `get`/`rationale` (sub-gap (d), shared
contract surfaces, is fixed on main).

## Evidence (verified on main, 2026-07-12)
- (a) `cairn get <node>` omits accepted-decision pointers; `--json` returns
  `"decisions": null` while `cairn decisions <node>` lists them.
- (b) `cairn rationale <node>` silently includes neighbour-node decisions
  without flagging them as transitive (scratch probe: dec on an inbound
  neighbour appeared in the queried node's rationale).
- (c) Fully-qualified IDs required everywhere; short aliases rejected with
  `CAIRN_QUERY_NODE_NOT_FOUND` (though the error suggests the full ID).

## Task
One todo per issue (gh:#239 is itself a bundle); track sub-items here:

- [x] (a) include accepted-decision pointers in `cairn get`
- [x] (b) label neighbour-sourced decisions in `cairn rationale`
- [x] (c) accept unambiguous suffix aliases in node lookup

## Resolution (2026-07-16)

All three gaps shipped on `vibe/query-ergonomics`:

- (a) `cairn get` now carries accepted-decision pointers: the `--json`
  payload gains a `decisions` array of accepted decision IDs (shared
  helper `accepted_decision_ids` in `src/query_api/serialise.rs`, wired
  in `src/query_api/mod.rs`), and the human rendering appends an
  "Accepted decisions:" section (`src/cli/render/node.rs`).
- (b) `cairn rationale` labels neighbour-sourced decisions as
  transitive: each such decision object gains a `via` array naming the
  neighbour node IDs it arrived through
  (`src/query_api/handlers/node.rs`), and the human rendering appends a
  "(via <node>)" suffix (`src/cli/render/artefacts.rs`).
- (c) node lookup accepts unambiguous dotted-suffix aliases at the
  shared seam `Graph::resolve` (`src/map/graph.rs`), so every
  node-taking command benefits. An ambiguous suffix fails with an error
  listing the candidate IDs; exact IDs behave exactly as before.

Tests: `src/query_api/tests.rs` (get pointers),
`src/query_api/handlers/node.rs` (rationale via labels),
`src/map/graph.rs` (suffix aliases), plus human-render tests in
`src/cli/render/node.rs` and `src/cli/render/artefacts.rs`. The
`api_node_app_api` wire snapshot was updated deliberately for the
additive `decisions` field.
