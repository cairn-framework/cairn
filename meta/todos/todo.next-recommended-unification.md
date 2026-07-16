---
node: cairn.kernel.query
status: open
created: 2026-07-16
---

# Next Recommended Unification

`cairn status` disagrees with `cairn next` about what to do next.
`status_json` computes next_recommended from the beads backlog only
(`src/query_api/handlers/project.rs:31`), ignoring findings and native
todos, while `cairn next` implements the correct priority order
(`src/cli/render/remediate.rs:100`: findings first via top remediation
action, then top open native todo, then top ready bead, per
dec.native-todos-first). An agent asking "what is the state" gets a
different answer than one asking "what should I do next". Distinct from
the fixed `todo.status-active-changes-bug`, which covered active_changes.

Fix in two steps:
1. Make status_json's next_recommended delegate to the same selection
   logic `cairn next` uses (extract the selection into a shared helper;
   render layers stay separate).
2. Consider a shared work-item projection (source, title, node, command,
   rank) across remediation actions, native todos, and beads in
   status/next/remediate --json, so agents see one queue vocabulary.
   Findings stay ephemeral and todos stay durable; only the presentation
   unifies. Materializing findings as durable todo artefacts was
   assessed and rejected (desync risk; findings are the error signal in
   the controller framing).

Motivation: `res.a2ui-analysis`, follow-on findings section (findings as
tasks). Step 1 is a small correctness fix, no proposal needed. Step 2
touches the external JSON contract; propose before implementing, and
coordinate with `todo.wire-format-schemas` so the projected shape ships
with a schema.
