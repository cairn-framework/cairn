---
node: cairn.kernel.cli
status: open
created: 2026-07-16
---

# Agent Effectiveness Benchmarks

Design and run A/B benchmarks measuring agent task performance with
cairn queries (context, get, neighbourhood, and locate once it exists)
versus without (tree plus grep only), on two or three pinned-SHA
open-source repos. Report files opened, tool calls, tokens, cost, and
time-to-correct-file per condition.

Per `res.codeatlas-analysis` (finding 2, verified): CodeAtlas's
`benchmarks/` reports are its most persuasive product surface, including
an honest partial-failure case. Copy the transparency, fix the method
gaps they left: multiple trials per condition, a described run harness,
variance reporting. The existing autoresearch webui eval harness measures
UI rendering quality and does not cover this.

Validate scope first (which tasks, which repos, harness design) before
committing to a change proposal; this is a large item.
