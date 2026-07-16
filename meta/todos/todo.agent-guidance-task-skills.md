---
node: cairn.kernel.cli
status: open
created: 2026-07-16
---

# Agent Guidance: Task-Shaped Skills and Guardrails

Upgrade cairn's agent-facing guidance per `res.codeatlas-analysis`
(findings 5, 6, 8, 10; all verified with corrections). Four bundled
pieces, all touching agent_guide.md and the installed skills:

1. **Task-shaped skills.** Expand cairn-dev's thin "Graph navigation
   patterns" section into full task-shaped playbooks in the CodeAtlas
   style: bug investigation, refactoring, architecture discovery, feature
   implementation, each mapping the activity onto exact cairn query
   sequences.
2. **Map-reading guardrail.** Add an explicit rule: map.json and map.md
   are generated snapshots for git-diff review, never agent context; use
   `cairn get` / `cairn neighbourhood` / `cairn files` / depth-capped
   `cairn context` instead; `cairn context --json` is a tooling escape
   hatch, not orientation.
3. **Not-found recovery ladder.** Document the recovery sequence for a
   failed node lookup (exact id, trust the error's suggestions and suffix
   aliases, then path-based lookup, then filesystem search). The resolver
   already does most of this (`src/map/graph.rs`); only the guidance is
   missing. Optional polish: edit-distance fallback for typo-class
   misses.
4. **Span-windowed reads.** Instruct agents to read only the located
   symbol's span. SymbolRecord already carries line and end_line, but
   end_line is only exposed in `--json`; add it to the text render of
   `cairn get <node> --symbols`.
