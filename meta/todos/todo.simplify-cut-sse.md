---
node: cairn.root
status: done
created: 2026-07-06
---

# Cut the SSE Spike

Part of todo.simplify-architecture (wave 1). Depends on: nothing.
Follow the shared rules in todo.simplify-architecture.

`src/sse.rs` (372 LOC) has zero internal callers; `src/lib.rs:48` calls it
an orchestrator integration spike. The node description says "Minimal SSE
consumer for Gas City integration".

Step 0, before deleting: confirm the Gas City integration is not an active
external consumer. Check `meta/research/gas-city-cairn-integration/` and
any Gas City decision for a live dependency on this module. If a real
consumer exists, convert this task to "document and contract the SSE
surface" instead and note that here.

If confirmed dead:

- Delete `src/sse.rs` and its `pub mod sse` line in `src/lib.rs`.
- Delete `meta/contracts/sse.md` and the `cairn.sse` node (and any edges)
  from `cairn.blueprint`.
- In the same change, re-anchor THIS todo's frontmatter to
  `node: cairn.root`: done todos stay in meta/todos/, and a todo
  pointing at a deleted node trips `CAIRN_TODO_ORPHAN_NODE`
  (`src/artefacts/registry/validate/mod.rs:36-45`), which would fail
  this task's own strict-scan acceptance.
- Run `cairn scan` to confirm no orphan or ghost findings appear.

Coordination note: this task and todo.simplify-persist-helper both edit
`src/lib.rs`'s module list; order-independent, but expect a trivial
conflict if branched concurrently.

Acceptance: `cargo test` green, `cairn scan --strict` clean, no reference
to `sse` remains under `src/` or `cairn.blueprint`.

Resolution (2026-07-07): confirmed dead and cut. Step 0 evidence: zero
internal callers (only the `pub mod sse` export in `src/lib.rs`); the
gas-city research made no code changes; issue #101 (SSE consumer spike,
part of epic #95) is aspirational, not a live consumer; and the spike's
direction was cairn-as-client of Gas City's stream, so no external system
links `cairn::sse`. `dec.close-blueprint-drift`'s "consumed by the Gas
City adapter" claim (2026-06-03) was already falsified by the 2026-07-06
four-audit investigation ratified in todo.simplify-architecture. Deleted
`src/sse.rs`, the `pub mod sse` line, `meta/contracts/sse.md`, and the
`cairn.sse` blueprint node (no edges referenced it); re-anchored this todo
to `cairn.root`.
