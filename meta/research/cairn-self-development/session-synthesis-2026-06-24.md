---
title: Cairn self-development synthesis
date: 2026-06-24
status: working synthesis (raw material for proper research artefacts + ratified decisions; NOT itself ratified)
note: This is a slate, not a decision. Items marked DECIDED are candidates for ratified meta/decisions entries. Items marked OPEN need the maintainer's call.
---

# Cairn self-development: session synthesis (2026-06-24)

## 0. How to read this

Every item is tagged with its state so decided direction is never confused with what is merely built today.

- **DECIDED**: we reasoned to this across the session and it holds. Candidate for a ratified decision.
- **TODAY**: verified code reality (what is actually built right now).
- **GAP**: the difference between DECIDED and TODAY. This is the work.
- **OPEN**: genuinely unresolved. The maintainer owns the call.

The single most important correction this turn: **cairn is native-first and zero-install by default. Beads is an optional backend, never the default.** Earlier phrasing that leaned on "beads" as the coordination layer was loose shorthand for what is built today, not what was decided.

## 1. Essence (immutable)

Cairn IS: a typed structural **map** of a system (nodes + edges) + **intent** per node (contracts) + **provenance** (decisions) + **drift detection** via interface-hash (a code-specific reconciler) + history/sync via git + orientation queries.

Identity: the self-honest, provenance-bearing **orientation map**. As multi-agent coding scales, the scarce resource is a shared, honest, queryable picture of "what exists, what it is for, why, and what is safe to change." That is cairn's wedge.

Everything else (git, code, beads, Gas City, hosting) is **swappable substrate** around that core, not part of it.

## 2. The layered model (DECIDED)

Three layers that **compose, not compete** (from `dec.no-orchestrator`):

- **Layer 3, orchestration (optional):** Gas City / Claude Code / any loop. Cairn rides an existing orchestrator; it does not build its own.
- **Layer 2, cairn's semantic lane:** blueprint, typed artefacts, two-chain authority (`decision -> blueprint -> contract -> code`), reconciler, drift gate. This is cairn's own thing and is always canonical.
- **Layer 1, pluggable storage (StateBackend):** default filesystem; native embedded coordination store (to build); hosted opt-in; beads/Dolt as a legacy optional backend.

"One point of contact" means a **porcelain over the engine** (the git analogy: you put one clean surface over git, you do not fold git into your app). It does NOT mean collapsing the layers or building cairn on top of beads.

## 3. cairn and beads, the clarification (own the contradiction)

- **DECIDED:** `install cairn` must be the **whole product**. The coordination store is **embedded in the binary**. Beads is an **optional extra, never required, never the default.**
- **The dominant constraint that forces this:** zero-install-anywhere. Claude Code Cloud, ephemeral agent sandboxes, and CI cannot install a Go binary (bd) or stand up a Dolt server. Anything that *requires* installing an external task engine is dead on arrival in the environments cairn most needs to live in. That single fact demotes bd from "default" to "one optional backend for people who already run it."
- **Beads keeps genuine separate value** (Dolt cell-merge across divergent clones, federation, orchestrator-independence, works standalone). So it stays as an optional backend / import-export bridge. It is not the engine cairn sits on.
- **Refinement of the earlier "porcelain over beads" framing:** cairn is a porcelain over a **pluggable backend** (filesystem/embedded by default; bd is one option), not specifically over beads.

## 4. Coordination store: the per-field split, trilemma, and engine (DECIDED direction; engine OPEN)

**The content/coordination split is PER-FIELD, not per-record:**

- **Content fields** (body, title, decisions, acceptance criteria, deps): files + git canonical, with a SQLite/index as a **derived read-cache**. Row-level git merge for free, readable, zero-install.
- **Coordination fields** (status, claim/assignee): **store-canonical**. Not text, not derived. They need a transactional store.

**Why text cannot hold status (3 reasons):**
1. A derived cache cannot be authoritative: rebuild-from-files overwrites the claim.
2. A jsonl write-lock only serializes writes; a conditional claim (`WHERE status='open'`) needs a real transaction, not a hand-rolled one.
3. Cross-device status-as-text causes a same-line conflict on the hottest field, which stalls unattended loops.

**The status-field trilemma (pick two of three):** {in-git + readable} / {race-safe} / {no-merge-engine}.
- Text = in-git + readable, NOT race-safe.
- SQLite = race-safe local, but status leaves version control (cross-device then needs CRDT/Dolt/hosted).
- Dolt = all three, but pays the Go-binary/server cost (this is exactly what beads paid for).

**Merge-granularity ladder:** none (SQLite binary) < row (jsonl-per-line + git) < cell (Dolt, conflict-aware) < field-convergent/never-block (CRDT: Automerge / cr-sqlite).

**Concurrency is core, not an edge case.** Multi-agent systems and subagents writing one dev's data make concurrent writes first-class. Merge-safety is a non-negotiable, not "someday."

**DECIDED lean on the task model:** make cairn own the task model **natively now**: a canonical record with `cairn task add/rm/done` verbs writing through StateBackend, markdown as a rendered view, content-hash IDs, file-per-task versioned by git. Reach for a heavy merge engine **only if** real concurrency bites.

**OPEN, the engine choice for the embedded store:**
- **Baseline:** text-files + gix (pure-Rust git) + hash-IDs. Zero new engine, leans on git already present. Weakest under heavy concurrency.
- **CRDT:** Automerge 2.0. Production-ready Rust, conflict-free (never blocks, but can pick odd-but-deterministic results), single binary, kills tiering. You maintain a store.
- **DoltLite:** brand new (Mar 2026), immature, C/C++ + WASM + FFI, git-style cell-merge (conflict-aware). Spike only, gated hard on maturity.

**Backend rungs, one interface, capability scales (no two tiers at the surface):**
embedded (default, zero-install, git push/pull sync, works in cloud/CI) -> hosted opt-in (e.g. Railway, real-time, works in Claude Code Cloud as an API call) -> bd/Dolt-server (legacy, existing users only).

## 5. History engine per artefact (DECIDED)

- **Git for content** (decisions, task bodies, contracts): free history, diff, blame, time-travel, PLUS richer typed lineage edges (`supersedes`, `informed_by`, `revisit_triggers`) that a DB row-log cannot express.
- **DB-audit (embedded/Dolt) only for DB-resident state** (status) that otherwise has no git history.
- **Do not reimplement git log** for things git already shows (a single decision file's edits). Spend surface area only where git lacks the graph/semantic grain: fingerprint/structural diff between commits, node-scoped history, decision-lineage-with-time, drift provenance.
- **The graph is derived:** the scanner rebuilds nodes/edges in memory each scan (about 23 nodes today). The only persisted state is the fingerprint/interface-hash cache. Scale via incremental reconciliation (re-scan only changed-hash files) + a derived index. A real graph DB is justified only if cross-graph relational queries become the bottleneck, which is far off.
- **Storage model:** content-addressed fingerprint bound to git commits, NOT a versioned graph store. Graph-root = a Merkle tree over node fingerprints + edges + interface hashes, bound to the commit.

## 6. Today's code reality (VERIFIED)

- `StateBackend` enum `{ Filesystem (default), Beads }` is real and pluggable, but it persists **state only, not content**.
- `BeadsStateBackend` shells out to `bd` (`Command::new("bd")`) for create-epic, create-task-beads, claim, list.
- `cairn change new` (`src/cli/commands/change.rs:42-81`) **deterministically writes** `meta/changes/<id>/tasks.md` from `- [ ] Task` lines; if `state_backend == beads`, it mints one bead per line (markdown canonical, beads derived). The write -> parse -> pluggable-backend seam **already exists**, so native task verbs are a generalization of this function, not greenfield.
- Todo artefacts (`meta/todos/*.md`): `cairn todos <node>` is a **read-only list**; cairn reads them, does not mint them. (The earlier blanket claim "cairn never authors markdown tasks" was wrong; it is true only for Todo artefacts, not change tasks.)
- Default backend is filesystem; bd activates only if present/configured. Cairn **degrades gracefully without bd**: core graph, drift, reconciliation, decisions, and navigation are all bd-independent; only the orphan-bead check goes quiet; the backlog layer loses `cairn next`'s ranked selection.
- **Every artefact is node-bound** by integrity rule (todo: exactly 1 node; decision/research: at least 1; review: 1). There is no node-less artefact type.
- MCP server is query-only today (`allow_mutating_tools: false`). LSP is local-editor only.
- **The incoherence the user flagged:** cairn already drives bd under the hood for change creation, yet `docs/agent/cairn-dev-workflow.md` still tells agents to run `bd ready` for the backlog. That is the two-tier UX. The native task verbs fix it.

## 7. The gap (DECIDED minus TODAY = the build)

1. No native embedded coordination store exists yet. Build it (keystone, next-step #2).
2. `cairn task add/rm/done` verbs do not exist. Add them (generalize the `run_change_new` seam).
3. `cairn-dev-workflow.md` still routes agents to `bd ready`. Rewrite to cairn verbs.
4. MCP is read-only. Add mutating task tools so cloud agents can manage work over streaming MCP.
5. `cairn init` does not print next commands. Add it (discoverability).

## 8. Task model and the inbox/triage front door (DECIDED)

- A "task" in cairn is one of: a **ghost node** (new module to build), a **node-attached todo/change** (work on an existing node), or **node-attached research** (explore a known area). All node-bound.
- "ghost node + contract + `scan`" is a complete task lifecycle **for content** (definition + acceptance) only. It is NOT coordination (atomic claim, status under concurrency, structural leases). "You do not need beads" is true for content, false for concurrent claim/status.
- **Untriaged work** (a bug with unknown blast radius; an unscoped "let's look into X") is **pre-architectural**: it has no node yet, so it cannot be a native cairn artefact. It lives in an **inbox** until triage assigns a node. Triage = assign the node = the front door into cairn.
- **Inbox home:** the **native embedded coordination store** holds node-less inbox items (correcting the earlier "use beads"). Root-node-as-inbox is the pure-files fallback when no store is configured.
- **Bug nuance:** drift detection catches **contract** drift only, not logic bugs. A bug whose code still matches its contract is invisible to `cairn scan`, so bugs genuinely need an inbox/tracker, not just the graph.

## 9. Empirical adoption findings (VERIFIED)

Source of truth for agent behavior is the OMP session transcripts (`~/.omp/agent/sessions`), NOT `.beads/interactions.jsonl` (which logs bd mutations only). From 80 `/cairn-loop` sessions, 2026-06-23 to 24, on kimi-code:

- New commands were **adopted for orientation** (`cairn next` 133x, `cairn context` 118x) but did **not displace fallbacks**. Native-nav share stayed flat at about 37-43% across all quartiles. `cairn next/context` 251x vs `bd ready/list` 375x.
- Resume never adopted cairn: agents read `session-handoff.md` about 1x/session; `cairn status` 0x.
- `cairn decisions --grep` barely used (2x).
- **Insight:** capability does not equal adoption. Onboarding is a **habit-displacement** problem, not a feature-coverage one. Fix: `cairn init` prints the next 2-3 commands; cross-sell the map at the moment of need; lead with provenance (the unique value grep/git/issue-tracker cannot provide).

## 10. The four next steps (ranked, decided) + artefact mapping

1. **Prove the thesis: cross-language rebuild test.** Take one small library, `cairn init --from-code`, rebuild it in another language from the blueprint, check it passes the original's contract-tests. Highest signal, bounded, doubles as the killer demo. -> `research` + a todo. Not a ghost node.
2. **Make it run anywhere.** Native embedded zero-install store + per-field content/coordination split, beads demoted to optional. The keystone that unblocks cloud/CI agents. -> `research` -> **ratified decision** -> ghost node(s) for the embedded-store module + `cairn task` verbs.
3. **Close the discoverability gap.** `cairn init` prints next commands; cross-sell the map in command output. -> `change` + todos on existing cli/root nodes.
4. **Finalize identity + README + the gate-first vs map-first call.** -> **ratified decision** + doc (README plain-language pass already done).

Only #2 is "blueprint as task" in the pure ghost-node sense.

## 11. Open questions (maintainer owns)

- **Gate-first vs map-first identity.** README leads map-first today ("give it the map"). Gate-first reframes it as "your agents write code; cairn makes sure they write it in the right place."
- **Engine choice for the embedded store:** gix-baseline vs Automerge vs DoltLite-spike.
- **Spec amendment.** Routing canonical task content through a backend (native verbs) extends/reverses the read-only `beads-task-layer` ruling, so it needs a ratified decision and a spec section 8.2 amendment.
- **Reopen no-orchestrator?** Only if you want cairn to *drive* the loop. Today cairn supplies inject (`cairn context`) + gate (`cairn hook/scan`) = a membrane; driving the loop is a different decision. Name it explicitly if you want it.
- **Mutation porcelain** (`cairn close/create/update` forwarding to the backend): contested. Ship only if read-parity does not already dissolve the two-tool friction.
- **Blueprint-as-authored-source vs blueprint-as-inferred-cache.** If `--from-code` is the primary onramp, the blueprint drifts toward editable inference output rather than source of truth. Real tension.

## 12. What survives the contrarian pressure (echo-chamber guard)

- **The gate** (pre-commit drift/contract check). Hardest thing to replicate with "just grep + long context." Lead with it.
- **Contracts + decisions** (intent + provenance). The unique value existing tools cannot provide.
- **Structural leases / blast-radius coordination.** The genuinely novel primitive only the graph enables (claim a subgraph, get a conflict signal before writing).
- **Genuinely threatened:** the blueprint as a human-authored artefact (vs inferred); the "map" framing (floor-plan or gate may be the stronger pitch); the empty-room cold-start ROI (so `--from-code` must be excellent).
