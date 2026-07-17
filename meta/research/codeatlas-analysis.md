---
id: res.codeatlas-analysis
nodes:
  - cairn.kernel.query
  - cairn.kernel.cli
  - cairn.brownfield
sources: [src.codeatlas]
method: primary
date: 2026-07-16
---

# CodeAtlas comparative analysis: transferable lessons for cairn

Multi-agent analysis run 2026-07-16: three parallel deep-reads (CodeAtlas
internals, CodeAtlas agent-integration surface, cairn's own overlapping
capabilities), one synthesis pass producing twelve candidate learnings,
then one adversarial verifier per learning instructed to refute against
cairn's actual code and principles. Four learnings confirmed, six
weakened to corrected claims, two refuted. Refuted and weakened claims
are recorded here so they are not re-proposed from scratch.

CodeAtlas and cairn sit at opposite poles of the same problem. CodeAtlas
is a fully extracted, human-free, symbol-level index optimised purely for
agent navigation token economics. Cairn is a human-authored declaration
reconciled against code with enforcement gates. On extraction machinery
cairn is strictly stronger (in-process tree-sitter with real signatures
and end_line spans versus per-file ctags subprocesses with pattern-grep
visibility heuristics). The transferable material is one query gap and
CodeAtlas's product layer: agent behaviour shaping and measured evidence.

## Confirmed

### 1. Repo-wide symbol-to-location lookup (adopt, medium)

Cairn's highest-leverage gap. The query surface (42 tools in
`src/query_api/registry.rs`) answers "what symbols does node X have"
(`cairn get <node> --symbols`) but never "where is symbol X defined".
CodeAtlas demonstrates that reverse lookup is the single most valuable
agent query: all three benchmark reports attribute their 50 to 60 percent
exploration-step collapse to it. Cairn already pays the extraction cost:
`SymbolRecord{name, kind, signature, file, line, end_line}` is persisted
per node in map.json (`src/reconcile/symbol.rs:5-17`,
`src/scanner/snapshot.rs:52-53`). A `cairn locate <symbol>` query (CLI
plus MCP) is an index over data cairn already has, and the result can
carry the owning node id, so a symbol lookup also lands the agent on the
blueprint node, its contract, and decisions. CodeAtlas cannot do that.
Follow-up: `todo.symbol-locate-query`.

### 2. A/B agent-effectiveness benchmarks (validate-idea, large)

Cairn's core pitch (reliable context for coding agents) is asserted, not
measured. CodeAtlas shows even a modest three-task benchmark set with
pinned SHAs, ground-truth files, per-step logs, and token accounting is
persuasive, especially when it includes an honest failure case. Copy the
transparency, fix the method gaps: multiple trials per condition, a
described harness, variance reporting. The existing autoresearch webui
eval harness measures UI rendering quality, not agent navigation; it does
not cover this. Follow-up: `todo.agent-effectiveness-benchmarks`.

### 3. Anti-lesson: do not compress map.json keys (avoid)

CodeAtlas compresses (`n`/`t`/`l`/`f`) because its map is context
payload. Cairn's map.json is a committed, deterministic, human-diffable
measurement record (`dec.persistent-map-snapshot` rationale: visible
symbol/state/finding diffs on every PR). Verbose self-describing keys are
correct; compression would optimise for the map-reading anti-pattern the
guardrail below forbids.

### 4. Anti-lesson: do not adopt ctags-style external extraction (avoid)

When language-coverage pressure arrives, keep adding in-process
tree-sitter `LanguageSpec` entries (`src/reconcile/generic.rs`, roughly
80 to 120 lines per language) rather than shelling out to Universal
Ctags. CodeAtlas's ctags dependency is the source of most of its
fragility (formatting-sensitive visibility heuristics, unpopulated
signature field, per-file subprocess cost), and non-deterministic
external extraction cannot back cairn's interface-hash fingerprints.

## Weakened (corrected claims worth acting on)

### 5. Task-shaped agent skills (adapt, medium)

Cairn's installed agent surface (agent_guide.md plus cairn-dev,
cairn-explore, cairn-loop) is workflow-loop-shaped. The only task-shaped
material is the thin "Graph navigation patterns" section in cairn-dev.
CodeAtlas ships five task-shaped playbooks (bug investigation,
refactoring, architecture discovery, repository exploration, feature
implementation) each mapping a dev activity onto exact query recipes.
Expand cairn's patterns section into full task-shaped skills. Consistent
with `dec.agent-pack-packaging` treating skills as a distribution
surface. Follow-up: `todo.agent-guidance-task-skills`.

### 6. Map-reading guardrail (adopt, small)

No cairn document tells agents not to read map.json or map.md wholesale;
the agent guide and skills never mention them. CodeAtlas's CAUTION.md is
a one-rule skill: query the index, never load the map. Cairn's version is
a short note in agent_guide.md and the skills: generated snapshots are
for git-diff review, never agent context; use `cairn get`,
`cairn neighbourhood`, `cairn files`, and the depth-capped
`cairn context` instead; treat `cairn context --json` as a tooling
escape hatch, not orientation. No new summary mode is needed.
Follow-up: bundled into `todo.agent-guidance-task-skills`.

### 7. Token budgets for JSON query surfaces (adapt, medium)

Cairn already measures text-output token efficiency (char-level
measurement in `dec.context-edges`, non-growth tests in
`output_token_efficiency_status_brief.rs`, depth rollup in context rendering). The
uncovered surfaces are the JSON escape hatches: `cairn context --json`
(deliberately unbounded), `cairn get --symbols --json`,
`cairn neighbourhood --json`, and the map snapshot on a large brownfield
repo. Extend the existing practice to those with tested numeric budgets,
quantifying the spec's "tight defaults, heavy is opt-in" principle
(docs/spec.md line 759). Deferred: no todo yet; revisit when a large
brownfield dogfood repo exists to measure against.

### 8. Not-found recovery (small, mostly built)

`MapGraph::resolve` (`src/map/graph.rs:142-200`) already auto-resolves
unambiguous suffix aliases, lists candidates on ambiguity, and appends
substring-based suggestions to `CAIRN_QUERY_NODE_NOT_FOUND`. Remaining
gaps: typo-class misses get no suggestions (matching is
substring-containment only; an edit-distance fallback would cover them),
and agent_guide.md documents no recovery ladder for a failed lookup.
Follow-up: bundled into `todo.agent-guidance-task-skills` (guide part
only; edit-distance fallback is optional polish).

### 9. Init-time ignore scaffolding (adapt, small)

Already specified in docs/spec.md section 6.1 (init proposes an ignore
list, human confirms) but unimplemented. The lesson from CodeAtlas is
only prioritisation plus one implementation note: reuse the 15
`IGNORE_PATTERNS` heuristics in `src/brownfield/onboard.rs` as the shared
source of truth rather than a second hardcoded list. Also note CodeAtlas
resolves its ignore file against CWD (a bug); cairn's scan-time merge is
already correctly root-resolved (`src/scanner/config/mod.rs:100-105`).
Follow-up: `todo.init-ignore-scaffolding`.

### 10. Windowed reads on symbol spans (adopt, small)

SymbolRecord carries line AND end_line (strictly better than CodeAtlas's
fixed -20/+80 window), but end_line is only exposed in `--json`; the text
render omits it, and cairn-explore tells agents to "read the source files
directly" with no span discipline. Guidance change plus a small render
addition. Follow-up: bundled into `todo.agent-guidance-task-skills`.

## Refuted (do not re-propose)

- **Rescan/freshness rules for agents.** CodeAtlas has a stale-index
  problem because agents read a static artifact. Cairn architecturally
  does not: every query re-reconciles against the live filesystem
  (`query_api` load path runs a full parse plus reconcile per call).
- **Multi-harness skill installation.** Already validated and ratified
  (`dec.agent-pack-packaging`, 2026-07-13, informed by
  `res.agent-pack-packaging-survey`); open work is execution of
  `todo.agent-pack-implementation`, not re-validation.
