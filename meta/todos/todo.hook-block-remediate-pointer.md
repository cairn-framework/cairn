---
node: cairn.kernel.hooks
status: open
created: 2026-07-16
---

# Hook Block Remediate Pointer

When a hook blocks (`ExitDecision::Block`), the rendered output
(`src/hooks/render.rs::render_human_verbose` and the JSON path) lists the
blocking findings with no pointer to the remediation engine, so a blocked
agent hits a dead end unless it already knows `cairn remediate` exists.

Fix: append a footer line via the copy registry (for example key
`hooks.block-remediate-pointer`): "Run `cairn remediate --json` for
prioritized fix actions." Optionally follow up by embedding
`remediate_json`'s actions filtered to the blocking findings' codes into
the hook JSON output, reusing the existing engine
(`src/query_api/handlers/remediate.rs`) rather than adding any new hint
table (a duplicate table was proposed and refuted, see the research).

Motivation: `res.a2ui-analysis` finding 1. A2UI standardizes
machine-readable errors specifically so the LLM can self-correct; cairn
has the engine but the gate surface does not advertise it.

Adjacent: `todo.remediate-copy-centralisation` (the footer should use the
copy registry from day one). Small enough to not need a change proposal.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It keeps hook failures pointed at actionable remediation.
