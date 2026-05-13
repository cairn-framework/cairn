# Gas City / Beads / cairness — Deep Analysis

**Date:** 2026-05-13
**Session:** Claude Code (Opus 4.7, 1M context) on branch `claude/gas-city-cairn-analysis-swwxw`
**Method:** Live inspection of cloned repos (`gastownhall/gascity`, `gastownhall/beads`), reading of full architecture docs, Yegge's "Welcome to Gas City" blog (user-supplied verbatim transcript; original is paywalled), and supplied cairness issue inventory.

All citations below are repo-relative paths to files inspected in-session against `main` at session time. If this analysis is later promoted to a CAIRN Source artefact, pin the inspected commits explicitly.

---

## 1. Gas City: what it actually is

### 1.1 The model

Per `gascity/engdocs/architecture/nine-concepts.md`, Gas City is five primitives + four derived mechanisms:

**Primitives (Layer 0-1):**
1. **Session** — start/stop/prompt/observe sessions regardless of provider
2. **Bead Store** — universal persistence substrate; everything is a bead
3. **Event Bus** — append-only pub/sub log
4. **Config** — TOML with progressive activation
5. **Prompt Templates** — Go text/template in Markdown

**Derived (Layer 2-4):**
6. Messaging (mail + nudge)
7. Formulas & Molecules (declarative workflows + runtime instances)
8. Dispatch (Sling) — agent + formula + molecule composition
9. Health Patrol — supervision, reconciliation, crash quarantine

### 1.2 The controller loop

`gascity/cmd/gc/controller.go:226` defines `controllerLoop()`. Each tick (default 30s):

1. **Dirty check** — fsnotify-driven config reload via `tryReloadConfig()` at `gascity/cmd/gc/controller.go:137`
2. **`buildAgents(cfg)`** — evaluates pool `check` commands in parallel, applies suspensions, resolves fixed agents
3. **`reconcileSessionBeads()`** — declarative convergence between session beads and running sessions; see `gascity/cmd/gc/session_reconciler.go`
4. **`wispGC.runGC()`** — purges expired molecules per TTL
5. **`orderDispatcher.dispatch()`** — trigger-conditioned formula/exec dispatch

Configuration drives everything. From `gascity/engdocs/architecture/controller.md` §Invariants:
> *"No role names in Go code. The controller operates on resolved config, runtime session names, and provider state."*
> *"SDK self-sufficiency: All controller operations function with only the controller process running. No user-configured agent role is required for any infrastructure operation."*

### 1.3 What "drift detection" means in Gas City

`gascity/engdocs/architecture/controller.md` interactions table:
> *"`internal/runtime` | `Provider` interface for Start/Stop/IsRunning/ListRunning/Interrupt/Peek/SetMeta/GetMeta/ClearScrollback. `ConfigFingerprint()` drives drift detection."*

`gascity/internal/runtime/fingerprint.go` is *"`ConfigFingerprint()` (SHA-256 of command + env + extras for drift detection)"* — drives agent restart when running instance's command/env diverges from declared config.

Other drift usages in repo grep:
- `gascity/release-gates/ga-9shf-gate.md` — `gc doctor` drift detector for Dolt port mismatches
- `gascity/plans/archive/huma-openapi-migration*.md` — CI gate ensuring committed OpenAPI spec matches code

**No drift concept between declared system architecture and actual code.** Verified by grep across `gascity/engdocs/`, `gascity/specs/`, and `gascity/internal/` for: `ontolog`, `blueprint`, `interface.hash`, `provenance`, `authority`. Only stray hits (e.g. `gascity/AGENTS.md`: *"The architecture docs are a reference, not a blueprint"*), never as an architectural primitive.

### 1.4 Out of scope by Gas City's own declaration

`gascity/specs/architecture.md` §7 explicitly excludes declarative schema specifications and framework positioning. Gas City is a control plane, not a framework.

### 1.5 Runtime providers — leanness confirmed

Per `gascity/engdocs/architecture/session.md`, providers include:
- `tmux` — primary interactive
- `subprocess` — local non-interactive
- `exec` — script-backed
- `k8s` — pod-backed
- `acp/auto/hybrid` — routing layers

An "agent" is whatever you put behind a `runtime.Config` (command, env, cwd). Bare Python scripts, Go binaries, curl calls, MCP clients — all work. Nothing forces Claude Code or any heavy harness. Confirms that the leanness concern that motivated cairness's lightweight agent spec is already addressed in Gas City as a first-class case.

---

## 2. Beads (MEOW substrate): what it actually is

### 2.1 Standalone, orchestrator-independent

Per `gastownhall/beads/README.md`:
> *"Beads is a CLI tool you install once and use everywhere. You don't need to clone this repository into your project."*

Installation: `brew install beads` / `npm install -g @beads/bd` / curl script. `bd init` initializes in any project; no orchestrator required.

### 2.2 The Bead schema

`gascity/internal/beads/beads.go:Bead`:
```go
type Bead struct {
    ID           string
    Title        string
    Status       string   // "open", "in_progress", "closed"
    Type         string   // "task" default; matches bd wire format
    Priority     *int
    CreatedAt    time.Time
    Assignee     string
    From         string
    ParentID     string   // step → molecule
    Ref          string   // formula step ID or formula name
    Needs        []string // dependency step refs
    Description  string
    Labels       []string
    Metadata     map[string]string
    Dependencies []Dep
}
```

`Type` is a free-form string. Beads persists; CAIRN would interpret.

### 2.3 Hash IDs and Dolt backing

Beads README §"Zero Conflict":
> *"Hash-based IDs (`bd-a1b2`) prevent merge collisions in multi-agent/multi-branch workflows."*

Beads README §"Features":
> *"Dolt-Powered: Version-controlled SQL database with cell-level merge, native branching, and built-in sync via Dolt remotes."*

Federation via Wasteland is built on Dolt-remote sync; orchestrator-independent.

### 2.4 MEOW is not a library

`gascity/AGENTS.md` verbatim:
> *"a thin layer atop the MEOW stack (beads → molecules → formulas)."*

MEOW = Beads (storage) + Molecules (formula instances, in gascity) + Formulas (TOML workflow definitions, in gascity). **Only Beads is independently installable.** "MEOW stack" describes the conceptual sandwich; not a downloadable package.

---

## 3. Gas City's API surface

Per `gascity/engdocs/architecture/api-control-plane.md` §1:
> *"Two architectural themes run through everything below: 1. The object model is the center; the CLI and the HTTP + SSE API are projections over it. One canonical domain, two typed surfaces. 2. Typed data end-to-end. Go structs with annotations drive a generated OpenAPI 3.1 contract."*

**Surfaces:**
- CLI (`gascity/cmd/gc/`) — broad subcommand set
- HTTP + SSE generated via Huma from typed Go structs
- Generated Go client for cross-process calls
- SSE event stream for long-running ops: 202 + `request_id` + `request.result` event

**Extension points for external integrators:**
- **Packs** — declarative agent topologies as TOML + prompts + formulas
- **Formulas** — `*.formula.toml` workflow definitions
- **Prompt templates** — Go text/template in Markdown
- **Runtime providers** — tmux/subprocess/exec/k8s/acp
- **`exec.Store`** — `provider = "exec:<script>"` delegates bead-store ops to user script

The canonical Gas Town topology itself ships as a pack (`gascity/examples/gastown/`). Per `gascity/examples/gastown/SDK-ROADMAP.md`: *"~1,200 lines of Go to make Gas Town run as pure configuration."* Even Gas Town is just-a-pack.

---

## 4. Cairness scope vs Gas City overlap matrix

Based on cairness issues #1, #2, #6, #7, #9, #10, #14 (supplied by user; repo `george-rd/cairness` is private). Coverage assessment against Gas City code and docs read in-session:

| Cairness issue | Scope | Gas City equivalent | Verdict |
|---|---|---|---|
| **#1** Epic: Grapharness | Lightweight harness-agnostic agent orchestration on CAIRN graph; <5MB Rust; 2-3k LOC | Full control plane in Go | **Standalone form duplicative.** Salvage: graph-walking scheduler concept (~400 LOC) → Gas City formula |
| **#2** Flow engine + YAML DAG, 500-700 LOC | YAML step DAG with conditions, retries, actions | Formulas + molecules (TOML + bead trees) | **Duplicated** |
| **#6** Adapter registry, 200+150/adapter | YAML adapter contracts for jcode/CC/litellm/codex | Runtime providers + prompt templates | **Mostly duplicated.** Per-harness glue lives in packs |
| **#7** Wave scheduler walking CAIRN graph, 400-500 LOC | Walk CAIRN graph, group into parallel waves, apply policy | Controller is config-driven, not graph-driven | **Not duplicated.** Real novel piece |
| **#9** Stats + dashboard + self-improvement, 1150 LOC | SQLite metrics, TUI/web dashboard, analysis agents propose flow changes | Event bus + Dolt audit | **Data layer duplicated.** Self-improvement loop novel |
| **#10** YAML flows vs CAIRN primitives | Architecture decision parked | — | Decision becomes: orchestrator-agnostic CAIRN with optional Beads backend |
| **#14** SQLite cache + DB state (closed-source) | CAIRN open-source file-based, cairness closed-source DB-backed | Dolt via Beads | **Dolt strictly better than SQLite** for versioning/branching/federation |

**Estimated overlap:** ~70%. Two novel pieces (#7, #9) survive but are formula-sized (hundreds of LOC), not standalone-orchestrator-sized (thousands).

### 4.1 Where the surviving novel pieces actually live

**Cairness #7 (graph-walking wave scheduler) splits across the two sides of the integration:**

- **CAIRN side (~50-100 LOC).** The graph-walking primitive — *"given the current change, what's ready right now?"* — must live where the graph definition lives. Concretely: `cairn query --ready --change <id> --json` walks blueprint + active change, applies `needs:` edge resolution, groups results by topological depth, emits waves as JSON. Covered by existing slate issues #4 (JSON contract), #9 (tasks-as-beads gives `bd ready` for free when beads-backed), and #3 (`ArtefactStore.query_by_dependency`).

- **Orchestrator side (~300-400 LOC, free re-use).** Wave dispatcher, concurrency limit, retry policy, role-based routing — these are operational, not architectural. Gas City already ships them via formula `needs:` edges, runtime pools, `max_restarts`/`restart_window`, label-based routing. In `adapters/gascity/` (issue #6) this becomes one formula (`cairn-wave-dispatch.formula.toml`) + a small worker prompt template.

Cairness was estimating 400-500 LOC because it was building the dispatcher from scratch. The dispatcher already exists in Gas City. We just need the right query feeding it. No new slate issue needed — the work is distributed across #3, #4, #6, #9.

**Cairness #9 (self-improvement loop)** is similarly distributed: Gas City + Dolt gives the audit data; the analysis-agent-proposes-changes loop is one or two formulas on top, also in `adapters/gascity/` or as a future skill. Defer until the data is flowing.

---

## 5. What CAIRN already has

Verified by source inspection in `~/cairn/`:

- `src/changes/` — change primitive with `artefact_ops.rs`, `types.rs`, `validate.rs`. Hooks for `CAIRN_CHANGE_ARTEFACT_CONFLICT` (`src/hooks/mod.rs:144`).
- `src/cli/accept.rs:run_accept_gate(change_id)` — apply/verify gate
- `src/cli/commands.rs:run_archive_command` — archive command
- `cairn.kernel.changes` module declared in `cairn.blueprint`
- Spec §9 — change directories, delta semantics (ADDED/MODIFIED/REMOVED/RENAMED), archive operation
- Spec line 178 — planned location `./meta/changes/`
- Spec §4 verbatim:
  > *"Cairn and OpenSpec solve different problems (OpenSpec is a change-lifecycle workflow, Cairn is a structural reconciliation framework), but OpenSpec's change-isolation and delta-merging patterns are directly applicable and are adopted in sections 9 and 12. **Cairn deliberately does not adopt OpenSpec's workflow layer**; the two tools are complementary and could coexist in the same repo."*

That non-goal needs amendment if openspec is to be retired entirely. See issue-slate.md #8.

---

## 6. What CAIRN does NOT have (openspec retirement gaps)

1. Conversational skills (`cairn-propose`, `cairn-explore`, `cairn-apply`, `cairn-archive`) — openspec's day-to-day value via `/openspec-propose` and friends.
2. `cairn change new <name>` scaffold with proposal.md / design.md / tasks.md templates.
3. In-change task tracking. OpenSpec has tasks.md; CAIRN doesn't yet. Beads with `parent=<change-id>` is the clean answer.
4. `cairn import-openspec` migration helper.
5. Registries as graph queries (currently `openspec/registries/*.md` as files).
6. Conventions surface (currently `openspec/conventions.md`; should be per-module `rules` blocks in `cairn.blueprint` or a top-level Source on `cairn.root`).
7. One-way switch: `openspec/changes/` → `meta/changes/`.

None of these are kernel-deep. Skills, scaffolds, a migration script, and small CLI commands. Reliable retirement is weeks of work, not months.

---

## 7. The structural argument

Three layers, three concerns:

```
Layer 3: Orchestration (optional)
   Gas City controller / sessions / packs / formulas
   CAIRN consumed as formula steps
   Future runners: adapters/<name>/

Layer 2: Semantic (CAIRN's lane)
   cairn.blueprint, typed artefacts, two-chain topology
   Reconciler, drift gate, interface hashes
   No equivalent in Gas City (verified by grep)

Layer 1: Storage (pluggable)
   Default: filesystem
   Optional: Beads (bd CLI / Dolt-backed)
   CAIRN trait: ArtefactStore
```

- Gas City: Layers 1 + 3, no Layer 2.
- CAIRN: Layer 2, pluggable Layer 1, externalised Layer 3.
- Beads: Layer 1 only.

These compose. They do not compete.

---

## 8. Yegge's framing (from supplied article transcript)

Direct quotes from the "Welcome to Gas City" Medium article (user-supplied verbatim; original at https://steve-yegge.medium.com/welcome-to-gas-city-57f564bb3607 is paywalled):

- *"Gas City has deconstructed the entire Gas Town stack into composable, declarative building blocks called 'packs'."*
- *"MEOW, the Molecular Expression of Work, is a lightweight Beads-based framework that places Work front and center, as the first-class system primitive, creating a versioned knowledge graph of all your issues and tasks."*
- *"every agent action recorded in a git-versioned Dolt database. That's your SOC2 story, sitting right there in the database, already written."*
- *"any agent can go temporarily insane, at any time, and make a bad call. No matter how smart they are."*
- *"To replace SaaS, you need the unglamorous stuff: declarative deploys, audit trails, version history, identity, and a memory layer that survives the inevitable agent failures."*
- *"Gas City is a high-control system. It has high parallelism... but it uses structure to keep agent swarms organized."*

These quotes establish that:

- Yegge's "knowledge graph" is the **work-as-graph** (beads with deps), not architecture-as-graph
- The reliability story is **probabilistic** (more agents reviewing each other), not deterministic (gate at commit)
- The pitch is **replace SaaS / business process automation**, not architectural governance

CAIRN's deterministic-gate-at-commit + architectural-truth angle is complementary, not competing.

---

## 9. Decisions reached this session

1. **Keep CAIRN.** Architecture-truth / typed-artefact / drift-gate / two-chain authority layer is genuine white space. Verified by grep of Gas City; no analogue.
2. **Retire cairness as scoped.** ~70% overlap with Gas City's mature surface. Salvage the graph-walking scheduler (~400 LOC) as a Gas City formula in `adapters/gascity/`.
3. **Retire cflx.** Was always experimental; CAIRN's `accept`/`archive` primitives plus an external runner replace it.
4. **Adopt Beads as a pluggable storage backend.** Optional but worth it: hash-IDs, Dolt versioning, federation via Wasteland, no orchestrator coupling.
5. **CAIRN does not ship its own orchestrator.** Integration with Gas City via a `cairn-gc` reference pack; future runners get their own adapter under `adapters/`.
6. **Retire `openspec/changes/`.** Move active phases to `meta/changes/` (already planned per spec line 178). OpenSpec workflow replaced by CAIRN skills + (optionally) beads-backed tasks.
7. **Amend spec §4** to reflect that workflow lives externally (skills + optional formulas), not as a CAIRN non-goal.

---

## 10. Honest limitations of this analysis

- The Medium article was paywalled; analysis used user-supplied verbatim transcript. Quotes are traceable to that transcript.
- `cairn` binary was not built in the session sandbox; analysis used grep/find/Read directly. A repeat with `cairn context` + `cairn neighbourhood` available would likely surface more.
- `bd` was not installed in the session sandbox; Beads claims were verified via README + cloned source inspection only, not via runtime use.
- cairness scope is from the issue inventory supplied by the user (`#1, #2, #6, #7, #9, #10, #14`). The repo `george-rd/cairness` is private; source not inspected.
- Gas City and Beads repos were cloned shallow (`--depth 1`) to `/tmp/gc-review/gascity` and `/tmp/beads-repo`. Tag/commit not pinned. If this analysis is promoted to a Source artefact, re-clone with explicit refs and re-verify.

---

## 11. The "graph IS orchestration" framing

Surfaced in conversation after the initial slate was drafted. Cairness #7 was reaching for this; the spec hints at it (line 71: *"Decisions can declare the blueprint nodes they apply to; the framework can then flag when a change to those nodes appears to violate the decision (v2 capability, deferred)"*).

Two distinct meanings:

**(a) Reactive: graph state changes drive work.** New `Todo` appears → worker spawned. `Contract` interface hash changes → drift gate fires. `Decision` flips to `accepted` → implementation work materialises.

**(b) Declarative: node types carry workflow semantics.** Each artefact type has an associated lifecycle and an associated kind-of-work. `Contract`: draft → reviewed → accepted. `Todo`: proposed → ready → claimed → done. The graph topology directly maps to dispatch decisions.

Both are CAIRN-side concerns. Neither requires CAIRN to own the dispatcher. The right division of labour:

- **CAIRN owns the semantics:** which node states imply which work types, what the lifecycle transitions are, when the drift gate must fire
- **The orchestrator owns the runtime:** parallelism, retries, pool scaling, crash recovery

This preserves the cairness vision in spirit (graph-native orchestration) while extracting the orchestrator into Gas City where it's more mature.

Three operational paths for graph-state-driven work in the Gas City world:

1. **CAIRN queries drive Gas City formulas.** `cairn query --ready --change <id>` returns ready wave; Gas City formula dispatches. Covered by #98 + #100.
2. **Beads-mediated.** Typed beads (`type=contract`) become work items via existing `bd ready` detection. Covered by #99 + #103.
3. **SSE reactive** (strongest form). CAIRN emits events on graph state changes; Gas City Orders react. Covered by #96 + #101.

**Gap in the current slate:** explicit `node-type → workflow` association in `cairn.blueprint`. Example: `Module @api → on_drift: cairn-drift-gate`, `Contract → on_status_change(accepted): cairn-implement`. The orchestrator becomes a dumb pump that runs whatever formula the graph state says is implied. This is the missing piece that makes "graph IS orchestration" concrete on the CAIRN side. Candidate for a new slate issue; pending decision.

---

## 12. Gas City tech-debt assessment

Asked late in the session because contributing back upstream became a strategic option. Concrete numbers from `/tmp/gc-review/gascity`:

| Signal | Value | Read |
|---|---|---|
| TODO/FIXME/HACK in non-test Go | 21 across ~250k LOC | 0.0084% density — well below industry concern |
| Test files | 796 | Heavy investment |
| Active design RFCs (`engdocs/design/`) | 20 | Working RFC pipeline; debt is documented before it's debt |
| Archived RFCs | 18 | Things actually ship and graduate |
| CHANGELOG detail | Per-fix operator-impact notes | Mature release engineering |
| Pre-commit hooks | Auto-regen OpenAPI + dashboard schema + lint + vet + test | CI-equivalent gates run locally |
| Recent activity | PR #1169 in last commit message | High velocity, large contributor base |

Sample TODOs read as `// Wired: TODO — operation context plumbing pending` — deliberate incremental implementation, not rot. No "broken and we don't know how to fix" debt visible.

`CONTRIBUTING.md` verbatim: *"Gas City is experimental software, but the repo is now structured for external contributors."*

**Verdict:** healthy. Contributing into Gas City would not be a rescue mission.

---

## 13. Contribution-path strategy

Key finding: `gastownhall/gascity-packs` exists as the explicit community pack home. README verbatim: *"A collection of opt-in Gas City packs... Packs compose through `pack.toml` imports, so a city can opt into any subset of the packs in this repo without forking."*

So the upstream contribution path is:

1. Build `adapters/gascity/` in `cairn-framework/cairn` (issue #100)
2. Dogfood locally for some weeks
3. Polish: docs, README, pinned Gas City version
4. Submit to `gastownhall/gascity-packs` as `packs/cairn-governance/` (or similar)
5. Optionally: a small PR to `gascity` core if a genuine integration-contract gap surfaces (unlikely; their extension model is good)

We're not contributing into 250k LOC of Go. We're contributing a pack (TOML + Markdown + prompt templates + a thin shim that shells to `cairn`). Tractable from outside; minimal upstream maintainer load.

**Community angle:** if `cairn-governance` lands in `gascity-packs`, CAIRN gets a discovery channel to ~15k-star Gas City community. The Gas City Discord audience (~2,000 active members per Yegge's article) is *exactly* the audience for architecture governance — people running multi-agent systems who've felt the hallucination pain and want deterministic gates. CAIRN repo stays the canonical home; the pack is the bridge.

Low-risk strategic bet. Downside is zero — you'd build the pack anyway for your own use under issue #100.
