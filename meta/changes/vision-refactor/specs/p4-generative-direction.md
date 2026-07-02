# Spec: Phase 4 — Generative direction

## Acceptance criteria

- `cairn bundle <synced-node> --json` returns contract body, decisions, and
  dependency interfaces for a node with a contract, decisions, and outbound
  edges.
- `cairn bundle` on a node with no contract returns `"contract": null` and
  names `"contract"` in a `missing` list; unknown node returns a standard
  `Finding` error.
- `cairn gap <node> --question "<text>"` creates
  `meta/decisions/gap-<node-slug>-<question-slug>.md` with the specified
  frontmatter; a second identical call gets a `-2` suffix; empty `--question`
  is refused.
- After a gap artefact exists, `cairn lint` reports exactly one
  `CAIRN_GAP_UNRESOLVED` warning; deleting the artefact returns lint to clean.
