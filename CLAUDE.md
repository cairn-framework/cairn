# CAIRN: Claude Code Working Notes

Repo-level context for Claude Code sessions working in this codebase. For the OpenSpec codex agent's instructions, see `AGENTS.md`.

## Coding behaviour

Activate the `karpathy-guidelines` skill (`.claude/skills/karpathy-guidelines/SKILL.md`) for any coding work in this repo: think before coding, simplicity first, surgical changes, goal-driven execution. Invoke it via the Skill tool when writing, reviewing, or refactoring code.

Activate the `cairn-dev` skill (`.claude/skills/cairn-dev/SKILL.md`) when navigating architecture, adding files, interpreting findings, or authoring blueprint/artefact changes. It covers the full cairn CLI surface, blueprint syntax, artefact schemas, and development loop.

## What CAIRN is

Connective tissue between past decisions, present code structure, and future intent. You author a declarative file describing your system; CAIRN parses it, reconciles against actual code, surfaces the graph + artefacts for coding agents to consume as reliable context, and gates commits when code drifts from the declaration. Extending beyond code to non-code domains (orgs, research, processes) is in-scope for future phases.

The framework's role (spec §3.4): *"a fence around the authority chain and a navigator for the provenance chain."*

## Where things live

| Path | What |
|---|---|
| `docs/spec.md` | Canonical spec. Read this first for any architecture question |
| `docs/design-system/` | Canonical design tokens, components, and live reference for any UI work |
| `docs/landing/` | Marketing landing page (GitHub Pages target) |
| `meta/changes/<phase>/` | Active phase proposals (proposal.md + design.md + tasks.md + specs/) |
| `archive/openspec/changes-archive/` | Archived phases (historical record; do NOT rewrite) |
| `archive/openspec/specs/<area>/spec.md` | Consolidated per-area specs |
| `docs/conventions.md` | Rust code conventions (error codes, module size, state versioning) |
| `docs/registries/` | `declared-items.md`, `error-codes.md` |
| `src/` | Rust implementation (phase 1+ kernel and onwards) |
| `src/ui_assets/` | Embedded web UI (styled via `docs/design-system/`) |
| `cairn.blueprint` | Root blueprint: CAIRN describing itself (dogfood) |
| `test/fixtures/cairn-bootstrap/` | Bootstrap fixture (test artifact, may lag behind root) |
| `meta/research/gas-city-cairn-integration/` | Gas City / Beads integration analysis, decisions, and issue slate |
| `AGENTS.md` | Agent orientation and workflow reference |

## Using cairn in this repo

This repo dogfoods cairn. The root `cairn.blueprint` describes the module graph. Prefer cairn CLI over grep/find for navigating architecture:

- `cairn scan` before committing: check for orphaned files or drift.

- `cairn scan --strict` for CI/agent verification gates (exits non-zero on Error or Warning findings).
- `cairn get <id>` to inspect a module (e.g. `cairn get cairn.kernel.scanner`).
- `cairn neighbourhood <id>` to see a module's dependencies and dependents.
- `cairn context` for a structured project overview (start here as an agent entry point).
- `cairn lint --json` for structured findings output consumable by scripts or agents.
- `cairn onboard` to group orphaned files by directory with ignore/node suggestions.
- `cairn ui --port 3000` to browse the graph in a browser (human use).

Module IDs follow dotted notation rooted at `cairn` (e.g. `cairn.kernel.map`, `cairn.reconcile`, `cairn.ui`). Run `cairn get <any-known-id>` to verify a node exists, or open `cairn.blueprint` to see the full ID list.

To develop cairn itself, follow the **Cairn Dev Loop** in `docs/agent/cairn-dev-workflow.md`: a repeatable orient, scope, propose, implement, verify, record, land sequence driven by cairn's own queries and gates. Invoke `/cairn-loop` to run one iteration.

When adding new source files or directories, check whether they fall under an existing module's `path` declaration in `cairn.blueprint`. If not, either add them to an existing module or declare a new one. A clean `cairn scan` (zero findings) is the target state.

## Architecture: two chains meeting at a hinge

CAIRN models **two chains**, not a flat six-layer stack:

- **Provenance chain** (evidence flowing in): Source → Research → Decision
- **Authority chain** (rules flowing out): Decision → Blueprint → Contract → Code
- **Hinge:** the Decision carries obligations in both directions

Describing CAIRN as "six layers" flattens the topology and loses the distinction between evidence and norms. v0.5 explicitly rejected the flat framing.

## Terminology state (as of 2026-04-20)

CAIRN spec is v0.7. **Phase 2.6 terminology rename has been applied and archived** (merge commit `3f15946`).

| Legacy (pre-phase 2.6) | Current (v0.7) |
|---|---|
| `DSL` / `.dsl` | `blueprint` / `.blueprint` |
| `ontology` | `map` |
| `index.md` (generated snapshot) | `map.md` |

If you see `DSL` or `.dsl` string literals in `src/cli/mod.rs` or `src/blueprint/parser.rs`, that is intentional legacy-file detection with a migration warning. Do not "fix" it.

**Everything else in v0.6 is kept deliberately.** Do NOT propose flattening the taxonomy; it encodes distinctions the framework depends on. Specifically:

- `reconciler` (pluggable interface), `scanner` (engine), `scan` (verb/CLI): three distinct concepts
- `artefact`: typed-schema kernel primitive (umbrella kept; direct types are contract, decision, todo, research, review, source)
- `rationale tension`: advisory non-blocking finding class, distinct from `interface contradiction` (blocking)
- `change` / `changes/`: carries delta semantics (ADDED/MODIFIED/REMOVED/RENAMED); `proposal.md` lives inside it
- `neighbourhood`: graph-theoretic query primitive
- `provenance chain` / `authority chain`: spec §3 spine
- `interface hash`, `ghost`/`synced`/`orphaned`, `drift`, `divergence`, `verified`/`external`/`unverified`, `hinge`: all kept

Use `blueprint` / `map` / `map.md` in all new prose, code identifiers, and spec drafts. The archived plan is in `archive/openspec/changes-archive/phase-2.6-terminology-rename/`; consolidated spec at `archive/openspec/specs/terminology-rename/spec.md`.

## Voice and audience

Em-dashes are banned in user-facing copy. Replace with period, colon, comma, or
parenthesis as context dictates. Full guidance: `docs/agent/voice.md`.

## UI and visual work

All UI changes (webui at `src/ui_assets/`, landing at `docs/landing/`, any new surface) must pull colors, type, spacing, radius, shadow, and motion from `docs/design-system/tokens.css`. Do not hardcode hex or rem values. Reuse classes from `docs/design-system/components.css` before introducing new components. Font authority is Source Serif 4 (headings, long-form) plus IBM Plex Mono (code, technical vocabulary) plus IBM Plex Sans (UI chrome). See `docs/design-system/README.md` for consumption patterns (marketing via `<link>`, Rust webui via `include_str!`). AGENTS.md carries the same directive for codex-driven phases.

## Workflow: cflx (Conflux)

Pre-commit hook runs `cargo fmt --check`. Verification gates: `cargo build` (zero
warnings), `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`.
cflx is retired per decision #105; existing `openspec/` phases are historical record.

## Workflow: Graphite (gt)

Full workflow in `docs/agent/graphite.md` (load when using `gt` or doing PR work).
Quick reference: `gt create -m "..."` to commit, `gt submit --stack --publish` to push.
Raw `git commit`, `git push`, `git checkout -b` bypass Graphite's metadata — use `gt`.

## Pre-submit review: mandatory

Before submitting any PR (via `gt submit` or `/forge-pr`), run both `/reforge` and `/debate` (or `/palantir-debate`) on the changes. This is not optional. The sequence is:

1. Implementation complete, tests pass, `cairn scan` clean
2. `/reforge` on the changed files (simplify, remove dead code, naming consistency)
3. `/debate` on the changes (adversarial review catches bugs, logic errors, convention violations)
4. Fix anything surfaced by reforge or debate
5. Then submit

This applies to every PR in a stack, not just the top. Skip only if the PR is a single-line documentation change.


## Debate format

When the user asks for a `/debate` or when a sign-off question merits one,
structure the response as three paragraphs:

**For** (one perspective). Steel-man the strongest argument in favour.

**Against** (the rival perspective). Steel-man the strongest counter-argument.

**Verdict** (decision plus reasoning). State the decision explicitly and
explain why it outweighs the opposing view.

Each paragraph should be self-contained. The Verdict paragraph must end with a
forced decision line, not a hedge.

## What cairn is, positively

Three principles in `docs/agent/principles.md` (load for architecture decisions).
Summary: typed artefacts encode obligations; authoring is template-driven and
tag-extensible; AI assists but the reconciler owns deterministic enforcement.

## What to avoid

- Rewriting archived phases under `archive/openspec/changes-archive/`; they are historical record.
- Inventing new terminology for concepts already named in the spec beyond the three applied renames.
- Describing the architecture as a flat six-layer stack.
- Calling `cflx` "cairn"; they're different tools. cflx is the workflow runner, cairn is the framework.
- Em-dashes in any prose in this repository.
- Hardcoded colors, sizes, or fonts in UI code when a design-system token exists.
- Skipping hooks. Forbidden bypass paths include `git commit --no-verify`, `git push --no-verify`, and the `SKIP=hookid` env var. If a hook fails, fix the underlying issue.

## Further reading

- `docs/spec.md`: canonical spec
- `docs/design-system/README.md`: UI token + component reference
- `archive/openspec/changes-archive/phase-2.6-terminology-rename/`: terminology rename plan and full rationale
- Archived phases for prior-art patterns: `archive/openspec/changes-archive/phase-2-artefacts/`, `archive/openspec/changes-archive/phase-2.5-graph-explorer/`

## graphify

This project has a graphify knowledge graph at graphify-out/.

Rules:
- Before answering architecture or codebase questions, read graphify-out/GRAPH_REPORT.md for god nodes and community structure
- If graphify-out/wiki/index.md exists, navigate it instead of reading raw files
- For cross-module "how does X relate to Y" questions, prefer `graphify query "<question>"`, `graphify path "<A>" "<B>"`, or `graphify explain "<concept>"` over grep; these traverse the graph's EXTRACTED + INFERRED edges instead of scanning files
- After modifying code files in this session, run `graphify update .` to keep the graph current (AST-only, no API cost)


<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:6cd5cc61 -->
## Beads Issue Tracker

This project uses **bd (beads)** for issue tracking. Run `bd prime` to see full workflow context and commands.

### Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --claim  # Claim work
bd close <id>         # Complete work
```

### Rules

- Use `bd` for ALL task tracking — do NOT use TodoWrite, TaskCreate, or markdown TODO lists
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

### Working with beads

- `.beads/issues.jsonl` is a passive export regenerated from the Dolt DB on every `bd` command. **Never hand-edit it.**
- After closing a bead, immediately re-export: `bd close <id> --reason "..." && bd export -o .beads/issues.jsonl`.
- Do not run bare `for ...; do bd close $id; done` loops. Each `bd` invocation re-imports the JSONL, which clobbers unexported closes from prior loop iterations.
**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.

## Agent Context Profiles

The managed Beads block is task-tracking guidance, not permission to override repository, user, or orchestrator instructions.

- **Conservative (default)**: Use `bd` for task tracking. Do not run git commits, git pushes, or Dolt remote sync unless explicitly asked. At handoff, report changed files, validation, and suggested next commands.
- **Minimal**: Keep tool instruction files as pointers to `bd prime`; use the same conservative git policy unless active instructions say otherwise.
- **Team-maintainer**: Only when the repository explicitly opts in, agents may close beads, run quality gates, commit, and push as part of session close. A current "do not commit" or "do not push" instruction still wins.

## Session Completion

This protocol applies when ending a Beads implementation workflow. It is subordinate to explicit user, repository, and orchestrator instructions.

1. **File issues for remaining work** - Create beads for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **Handle git/sync by active profile**:
   ```bash
   # Conservative/minimal/default: report status and proposed commands; wait for approval.
   git status

   # Team-maintainer opt-in only, unless current instructions forbid it:
   git pull --rebase
   git push
   git status
   ```
5. **Hand off** - Summarize changes, validation, issue status, and any blocked sync/commit/push step

**Critical rules:**
- Explicit user or orchestrator instructions override this Beads block.
- Do not commit or push without clear authority from the active profile or the current user request.
- If a required sync or push is blocked, stop and report the exact command and error.
<!-- END BEADS INTEGRATION -->
