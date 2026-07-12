---
node: cairn.kernel.query
status: open
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

- [ ] (a) include accepted-decision pointers in `cairn get`
- [ ] (b) label neighbour-sourced decisions in `cairn rationale`
- [ ] (c) accept unambiguous suffix aliases in node lookup
