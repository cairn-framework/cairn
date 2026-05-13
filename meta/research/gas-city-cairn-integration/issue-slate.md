# Issue Slate — Orchestrator-agnostic CAIRN with Gas City + Beads integration

**Date:** 2026-05-13
**Status:** Approved by user this session; pending creation in `cairn-framework/cairn` GitHub and/or local beads.
**Source of reasoning:** `./analysis.md` in this directory.

Each issue below is a self-contained body. Citations preserved inline so that whoever picks this up can audit the reasoning without re-reading the conversation.

The design principle running through all of them: **if Gas City gets replaced by orchestrator Y, only `adapters/gascity/` needs to change. Core + Beads adapter + CAIRN semantic layer stay put.**

---

## #1 — Epic: orchestrator-agnostic CAIRN with pluggable storage + reference adapters

**Labels:** `epic`, `phase:integration`
**Coupling:** meta

### Problem

CAIRN's distinctive value is the architecture-truth / drift-gate / typed-artefact / two-chain authority layer. Adjacent projects (Gas City, Beads) have matured to the point where building CAIRN-owned orchestration or CAIRN-owned graph storage would duplicate their work. But hard-coupling CAIRN to any one orchestrator would trap it. We need a three-layer architecture: semantic core, pluggable storage, pluggable orchestrator adapter.

### Evidence

- Gas City has zero architecture-as-data concepts. Grep of `gascity/engdocs/architecture/` returns hits for "drift" only in the runtime-config sense: `ConfigFingerprint()` per `gascity/engdocs/architecture/controller.md`, and in `gascity/cmd/gc/cmd_doctor_drift.go` for Dolt-port drift. No ontology, blueprint, interface-hash, authority-chain, or contract-vs-code drift concept anywhere.
- Gas City explicitly declares declarative schema specifications and framework positioning **out of scope** in `gascity/specs/architecture.md` §7.
- Beads is standalone and orchestrator-independent (Beads README: `brew install beads`, `bd init`, no Gas City needed).
- "MEOW" is not a library — it's Yegge's name for the stack. `gascity/AGENTS.md` verbatim: *"a thin layer atop the MEOW stack (beads → molecules → formulas)."*

### Scope

Track sub-issues #2 – #11. Maintain the layer boundaries (semantic / storage / orchestrator) as the architectural invariant.

### Acceptance

Sub-issues #2 – #5 are agnostic; #6 – #7 are the first reference adapter (Gas City); #8 – #10 retire OpenSpec; #11 records the no-orchestrator decision. A future runner Y can be supported by adding `adapters/Y/` without touching #2 – #5.

---

## #2 — Define and document the CAIRN integration contract for orchestrators

**Labels:** `orchestrator-agnostic`, `phase:integration`
**Coupling:** agnostic

### Problem

Any orchestrator that wants to drive CAIRN needs a stable surface: command shapes, JSON output schemas, exit-code semantics, an event envelope, and a subscription primitive. Today this is implicit, so any adapter risks being bespoke.

### Evidence

- Gas City's own success here is the model to copy. Per `gascity/engdocs/architecture/api-control-plane.md` §1: *"The object model is the center; the CLI and the HTTP + SSE API are projections over it. One canonical domain, two typed surfaces."* and *"Typed data end-to-end. Go structs with annotations drive a generated OpenAPI 3.1 contract."* Same separation belongs in CAIRN.
- Gas City formula step semantics require predictable exit codes and JSON for `needs:` resolution and `on_failure` branching (`gascity/engdocs/architecture/formulas.md` via index).
- Cairness issue #6 ("Adapter registry and harness contracts") motivated harness-side contracts. This issue is the symmetric problem: the contract CAIRN itself exposes to runners.

### Scope

- New doc `docs/integration-contract.md`: stable CLI surface, JSON schema per command, exit-code taxonomy, event envelope (JSONL on stdout as baseline), subscription primitive
- Exit codes: `0` clean, `1` advisory finding, `2` blocking finding, `3` fatal error
- Event types (initial set): `drift.detected`, `blueprint.changed`, `artefact.created`, `reconcile.complete`

### Acceptance

- A reader of `docs/integration-contract.md` can write an adapter without reading CAIRN source
- `cairn --help` matches the documented surface for every command
- Contract examples parse cleanly in CI

---

## #3 — Pluggable `ArtefactStore` trait with filesystem default

**Labels:** `orchestrator-agnostic`, `phase:integration`
**Coupling:** agnostic

### Problem

CAIRN's typed artefacts live as files today. To swap in Beads (or any future backend) without forking the core, persistence needs a seam.

### Evidence

- Direct template available in `gascity/internal/beads/beads.go`: a `Store` interface with four implementations (BdStore, FileStore, MemStore, exec.Store). Lets Gas City swap storage backends without touching consumers.
- Cairness issue #14 ("CAIRN optional SQLite cache + cairness database state") anticipated a pluggable backing store. This issue generalises: trait first, backends second.
- Filesystem stays the default → no regression for existing CAIRN users.

### Scope

- `ArtefactStore` trait in CAIRN core: `load`, `save`, `list`, `query_by_type`, `query_by_label`, `query_by_dependency`
- Default impl: filesystem (preserves current behaviour bit-for-bit)
- Backend selection via `cairn.blueprint` config and `--storage` CLI flag
- Trait-level test suite shared by all impls

### Acceptance

- All CAIRN commands pass on filesystem backend (no regression)
- New backends slot in via the trait alone; no changes to consumers

---

## #4 — Stable JSON output and documented exit codes across all `cairn` commands

**Labels:** `orchestrator-agnostic`, `phase:integration`
**Coupling:** agnostic

### Problem

Without stable JSON, every adapter scrapes stdout. Without documented exit codes, no orchestrator can branch on CAIRN results.

### Evidence

- Beads ships this as a headline feature (Beads README §"Features": *"JSON output, dependency tracking, and auto-ready task detection"*). It's why agents can compose `bd ready --json | jq | bd update --claim`. CAIRN should match.
- Gas City formula step dispatch reads exit codes for `needs:` resolution (`gascity/engdocs/architecture/formulas.md`). Predictable signals are CAIRN's responsibility, not the adapter's.
- Cairness #2 ("Flow engine and YAML step DAG") presumed clean step output — that's a CAIRN-side guarantee, not a runner-side wrapper.

### Scope

- Audit every `cairn` command. Define a JSON schema per command (linked from #2)
- `cairn lint`, `cairn scan`, `cairn reconcile`, `cairn neighbourhood`, `cairn get`, `cairn onboard`, `cairn context` all support `--json`
- Exit codes documented in `--help` and in `docs/integration-contract.md`

### Acceptance

- Schema-validated round-trip tests per command
- Exit-code taxonomy is exhaustive and tested

---

## #5 — Beads `ArtefactStore` backend + bead-type ↔ CAIRN artefact-type schema enforcement

**Labels:** `beads-adapter`, `phase:integration`
**Coupling:** beads (orchestrator-agnostic)

### Problem

Replicating Beads's Dolt-backed versioning, hash-IDs, and federation in Rust is years of work. Adopting it as a backend gets all of that for free. But beads stores untyped bytes per `type` string — CAIRN's value is in the typed *semantics*, which it must enforce on top.

### Evidence

- Beads is independent of any orchestrator (Beads README: *"Beads is a CLI tool you install once and use everywhere."*).
- Beads carries no Gas-City-specific assumptions. Per `gascity/internal/beads/beads.go`, the `Bead` struct is generic: `{ID, Title, Status, Type, Assignee, ParentID, Ref, Needs, Description, Labels, Metadata, Dependencies}`. Equally usable from Rust via the `bd` CLI.
- Hash IDs prevent merge collisions (Beads README §"Zero Conflict": *"Hash-based IDs (`bd-a1b2`) prevent merge collisions in multi-agent/multi-branch workflows."*) — a problem CAIRN's file-based artefacts will hit at scale.
- Federation via Wasteland is a Beads/Dolt property, not a Gas City property — works with bare `bd` (Beads README: *"built-in sync via Dolt remotes"*).
- Cairness #14 already identified Dolt as the right backend.
- From CAIRN's `CLAUDE.md` "What cairn is, positively" §1: *"Typed artefacts encode obligations, not labels. Each direct type (contract, decision, todo, research, review, source) has a different role in the two-chain topology. The kernel's enforcement value comes from those role differences."* The enforcement layer is CAIRN's job.
- Two-chain topology (provenance: Source→Research→Decision; authority: Decision→Blueprint→Contract→Code) maps to bead edges: `relates_to`, `needs`, parent (Beads README §"Graph Links": *"`relates_to`, `duplicates`, `supersedes`, and `replies_to` for knowledge graphs."*).

### Scope

- `BeadsStore : ArtefactStore` impl, talking to `bd` via subprocess initially
- Config: `[storage] provider = "beads"` in `cairn.blueprint`
- Schema per artefact type (required fields, status transitions, edge constraints)
- Validation on write through any `ArtefactStore` backend
- Edge mapping: provenance → `relates_to`; authority → `needs`; hinge → parent
- Round-trip tests: write through CAIRN, read via `bd`, validate

### Acceptance

- `cairn scan` writes typed artefacts to a `bd` store; `bd list --type=contract` returns them
- A bead with `type=contract` not authored by CAIRN can be detected as invalid and surfaced as a lint finding
- All six direct types have schemas + round-trip tests
- Works on a project that has only `bd` installed (no Gas City required)

---

## #6 — `adapters/gascity/` reference pack: formulas, prompts, install steps

**Labels:** `gas-city-adapter`, `phase:integration`
**Coupling:** gas-city

### Problem

Gas City is the highest-quality runner available. A first-class pack makes CAIRN adoptable by anyone running Gas City without bespoke wiring. Location under `adapters/<name>/` keeps the door open for additional adapters.

### Evidence

- Per `gascity/engdocs/architecture/glossary.md`, a pack is *"a reusable agent configuration directory loaded from `pack.toml`"* — the canonical Gas City extension surface.
- Gas City ships its own canonical topology *as a pack* in `gascity/examples/gastown/`, validated by `gascity/examples/gastown/SDK-ROADMAP.md`: *"~1,200 lines of Go to make Gas Town run as pure configuration."* Even Gas Town is just-a-pack.
- Yegge article verbatim: *"Gas City has deconstructed the entire Gas Town stack into composable, declarative building blocks called 'packs'."*
- Pack contents map cleanly to CAIRN commands: `cairn-reconcile.formula.toml` → `cairn scan && cairn lint`; `cairn-drift-gate.formula.toml` → `cairn lint --json` with exit code 2 → step failure.

### Scope

- `adapters/gascity/pack.toml` + `formulas/` + `prompts/`
- Formulas: `cairn-reconcile`, `cairn-lint`, `cairn-drift-gate`, `cairn-onboard`
- README with install steps and Gas City version pin

### Acceptance

- Single copy/symlink step makes `cairn` formulas available in any Gas City city
- Works against current Gas City release; pinned version documented

---

## #7 — SSE event consumer spike for reactive reconciliation

**Labels:** `gas-city-adapter`, `spike`
**Coupling:** gas-city (spike)

### Problem

Gas City emits a typed event stream over SSE. A CAIRN process that subscribes can react to bead lifecycle events without polling.

### Evidence

- Per `gascity/engdocs/architecture/api-control-plane.md` §2, every HTTP + SSE endpoint is generated via Huma from typed Go structs. Long-running mutations return 202 + `request_id` and emit `request.result` events on the SSE stream.
- Async pattern documented in `gascity/engdocs/design/async-request-result.md` (cross-referenced from api-control-plane.md).
- Yegge article: *"every agent action recorded in a git-versioned Dolt database. That's your SOC2 story, sitting right there in the database, already written."* The same stream powers audit *and* reactive integrations.

### Scope (spike-sized)

- Subscribe to `GET /v0/events`; print typed events; demonstrate re-running `cairn lint` reactively on bead lifecycle
- Decide critical-path inclusion after prototype

### Acceptance

- Working prototype documented at `adapters/gascity/SSE-spike.md`
- Decision recorded: ship / defer / drop

---

## #8 — Change-lifecycle skills + scaffold (`cairn-propose`, `cairn-explore`, `cairn-apply`, `cairn-archive`, `cairn change new`)

**Labels:** `openspec-retire`, `phase:integration`
**Coupling:** openspec-retire

### Problem

CAIRN has the change primitive (`src/changes/`), archive command, and accept gate. It lacks the conversational skills that make the lifecycle ergonomic — which is most of openspec's day-to-day value via `/openspec-propose`, `/opsx:apply`, `/openspec-archive-change`.

### Evidence

- CAIRN kernel already supports change directories: blueprint declares `cairn.kernel.changes` (cairn.blueprint Changes module), source at `src/changes/`. The framework piece is done.
- The spec deliberately scoped out workflow: `docs/spec.md` §4 *"Cairn deliberately does not adopt OpenSpec's workflow layer."* **This issue amends that:** skills are external to the kernel, so they don't violate the spec — the layer exists outside the framework, as Yegge's "prompt templates as primitive" pattern (`gascity/engdocs/architecture/nine-concepts.md` primitive #5).
- `src/cli/accept.rs:run_accept_gate` and `src/cli/commands.rs:run_archive_command` are the existing kernel-side hooks the skills will drive.
- Beads provides an `AGENTS.md`/`bd setup` pattern for agent discovery (Beads README §"Quick Start") — same pattern fits CAIRN skills.

### Scope

- Skills: `cairn-propose`, `cairn-explore`, `cairn-apply`, `cairn-archive`. Markdown frontmatter + body, drop into `.claude/skills/`
- `cairn change new <name>` CLI command: scaffolds `meta/changes/<name>/` with proposal.md, design.md, tasks.md templates
- Templates use ADDED/MODIFIED/REMOVED/RENAMED delta syntax already in spec §9
- Spec §4 amendment: change "Cairn deliberately does not adopt OpenSpec's workflow layer" to clarify the kernel doesn't ship a workflow; workflow is external (skills + optional formulas)

### Acceptance

- Each openspec skill has a CAIRN equivalent that drives the same lifecycle
- `cairn change new` produces a valid change dir; `cairn scan` accepts it
- Spec §4 amendment merged

---

## #9 — Tasks-as-beads inside a change

**Labels:** `openspec-retire`, `beads-adapter`
**Coupling:** openspec-retire

### Problem

OpenSpec's `tasks.md` checklist is the in-change task tracker. Two agents working the same change race on markdown edits. A graph store with atomic claims fixes this.

### Evidence

- Beads README §"Zero Conflict": *"Hash-based IDs (`bd-a1b2`) prevent merge collisions in multi-agent/multi-branch workflows."*
- Beads README §"Essential Commands": `bd ready` lists unblocked tasks; `bd update <id> --claim` atomically claims. Exactly the primitives needed for a tasks.md replacement.
- Bead `parent` and `needs` edges (`gascity/internal/beads/beads.go`: `Bead.ParentID, Bead.Needs`) express "task X belongs to change Y, depends on task Z."
- Beads supports hierarchical IDs for epics → tasks → sub-tasks (Beads README §"Hierarchy & Workflow"): `bd-a3f8` epic, `bd-a3f8.1` task. Maps naturally to change-as-epic, tasks-as-children.
- Filesystem fallback preserves CAIRN's standalone story per #3 (#5 backend is optional).

### Scope

- When `[storage] provider = "beads"`: `cairn change new` creates an epic bead for the change; tasks are child beads with `parent` and `needs`
- `cairn change tasks` lists ready tasks for current change
- `cairn change apply` claims and walks tasks
- Filesystem fallback: tasks.md as today, no atomic claim guarantee

### Acceptance

- Two agents on the same change cannot claim the same task
- `bd ready --label=change:<id>` and `cairn change tasks` show the same set
- Filesystem backend continues to work

---

## #10 — OpenSpec retirement: migration + parity

**Labels:** `openspec-retire`, `phase:integration`
**Coupling:** openspec-retire

### Problem

Today: `openspec/changes/` is live (phases 7.6, 7.7, 8, 9, 10 active + archive). `meta/changes/` is the CAIRN-planned location (`docs/spec.md` line 178). Registries and conventions live under `openspec/`. Retirement needs a one-way switch.

### Evidence

- `docs/spec.md` line 178: *"./meta/changes/ — Active change directories"* and 180: *"./meta/changes/archive/ — Merged changes, date-prefixed"*. Destination already chosen.
- Current openspec inventory: `openspec/{changes, specs, registries, conventions.md, config.yaml}` (verified via `ls openspec/`).
- Per-module `rules` blocks in `cairn.blueprint` are the spec-sanctioned home for project-specific conventions (`docs/spec.md` §6).
- Registries become graph queries: `declared-items.md` → `cairn query --tag declared`; `error-codes.md` → either a Source artefact or generated from code via the macro crate (`cairn-macros/`).

### Scope

- `cairn import-openspec` one-shot: walks `openspec/changes/`, recreates active phases under `meta/changes/`, copies archive preserving date prefixes
- Document conventions migration: `openspec/conventions.md` content moves into per-module `rules` blocks or a top-level Source artefact on `cairn.root`
- Document registry migration: queries that reproduce `openspec/registries/declared-items.md` and `error-codes.md` content
- Add `openspec/` to ignore list once frozen
- Update `CLAUDE.md` and `AGENTS.md` to point at `meta/changes/` and remove openspec skill references

### Acceptance

- After running `cairn import-openspec`, `meta/changes/` mirrors `openspec/changes/` semantically
- `cairn query --tag declared` returns the same set as `openspec/registries/declared-items.md` did
- `openspec/` can be deleted (or kept read-only for history) without losing information
- All openspec skill references in repo docs replaced with CAIRN equivalents

---

## #11 — Decision: CAIRN does not ship its own orchestrator; defer to external runners

**Labels:** `decision`, `epic`
**Coupling:** meta

### Problem

Cairness was scoped to be a lightweight orchestrator on top of CAIRN. After deep review of Gas City and Beads, an orchestrator built inside the CAIRN project would duplicate ~70% of Gas City's mature surface while losing the community / audit / federation benefits. Record this as a durable decision so the question doesn't reopen by accident.

### Evidence

- Gas City's controller is production-grade: declarative `city.toml`, fsnotify-driven hot reload, pool evaluation in parallel, crash quarantine with `max_restarts`/`restart_window`, graceful two-pass shutdown, single-controller `flock` on `.gc/controller.lock`, Unix-socket IPC (`gascity/engdocs/architecture/controller.md` §"Invariants").
- `gascity/examples/gastown/SDK-ROADMAP.md` shows the remaining work to express Gas Town fully as configuration is ~1,200 LOC of Go. A CAIRN-built orchestrator would be reinventing all of this.
- Beads provides distributed graph storage with hash-based merge-safe IDs, Dolt versioning, federation via Wasteland (Beads README). Reimplementation at that quality is years of work.
- Cairness epic #1 estimated 2,000-3,000 LOC for a standalone cairness. The components that survive contact with Gas City (the graph-walking scheduler from cairness #7) are ~400-500 LOC and can ship as a Gas City formula instead via #6 of this slate.
- The unique CAIRN value — typed artefacts, two-chain topology, drift gate, blueprint reconciliation — has **zero** analogue in Gas City. Grep of `gascity/engdocs/` for ontology / blueprint / authority / provenance returns only stray hits, never as an architectural concept.

### Decision recorded

- CAIRN does not own an orchestrator
- Orchestrator integration is the contract in #2 + reference adapters under `adapters/`
- Cairness as scoped is retired. The graph-walking scheduler may live on as a Gas City formula in `adapters/gascity/`
- cflx is retired in favour of CAIRN's own `accept`/`archive` primitives running under any external orchestrator (or none)

---

## Conversion notes

If seeding as **beads** (recommended given local dogfood):

```bash
# Epic
bd create "Epic: orchestrator-agnostic CAIRN with pluggable storage + reference adapters" \
  --type=epic -p 0 --label=phase:integration

# For each sub-issue:
bd create "<Title>" -p <priority> --label=<coupling-label> --label=phase:integration
bd dep add <sub-id> <epic-id>  # parent edge
```

If seeding as **GitHub issues** in `cairn-framework/cairn`: this slate is in submit-ready form. Title + body + labels can be lifted directly. Labels likely needed (proposed; check existing first): `epic`, `orchestrator-agnostic`, `beads-adapter`, `gas-city-adapter`, `openspec-retire`, `decision`, `phase:integration`, `spike`.

## Cross-references in this slate

- #1 ⇒ all
- #2 unblocks #5, #6, #7
- #3 unblocks #5
- #4 unblocks #6
- #5 unblocks #9 (storage backend for tasks-as-beads)
- #8 enables #9 and #10
- #11 references #1 (epic) and #6 (where the salvaged scheduler lives)
