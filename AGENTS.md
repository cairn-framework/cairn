# Cairn: Agent Orientation

Cairn is a graph-based architecture map for codebases: systems, containers, modules, and actors as nodes joined by dependency edges, each node carrying code targets, contracts, artefacts (decisions, todos, research), and temporal history. The graph is the source of truth for what exists, how it connects, and why it is shaped that way.

Two chains meet at a hinge: the **provenance chain** (evidence flowing in: Source → Research → Decision) and the **authority chain** (rules flowing out: Decision → Blueprint → Contract → Code). Describe the architecture through this two-chain topology, never as a flat stack of layers.

## Start here

Start at `.claude/skills/cairn-dev/SKILL.md`. It is a short router: it names the target-authority precedence, the first orientation query, and the gate, then points at the one reference your task needs. Load the reference the router sends you to, not all of them.

If your task has a change directory (`meta/changes/<change-id>/`), work from its `proposal.md` (why), `design.md` (how), `tasks.md` (what), and `specs/` (acceptance criteria).

## Where things live

| Path | What |
|---|---|
| `docs/conventions.md` | Rust conventions (error codes, module size, state versioning, testing, docs). Authoritative. Section 10 gives artefact placement; section 11 routes new prose to its owning layer, so check it before adding prose anywhere. |
| `docs/registries/` | `declared-items.md`, `error-codes.md`. Check when adding public items or error codes to avoid collisions. |
| `archive/openspec/changes-archive/<phase>/specs/` | Other phases' acceptance criteria. Relevant only when your design.md references them. |
| `archive/openspec/specs/<area>/spec.md` | Consolidated per-area specs, distinct from the per-phase criteria above. |
| `docs/spec.md` | Narrative model and history. Fallback for what the graph cannot answer; never bulk-loaded for routine work (`dec.spec-authority-retirement`). |
| `docs/design-system/` | Canonical tokens, components, fonts, and live reference for any UI work. |
| `docs/` | Marketing landing page (GitHub Pages target); pulls from the design system like any UI surface. |
| `cairn.blueprint` | Root blueprint: cairn describing itself (dogfood). The graph's source of truth. |
| `tests/fixtures/cairn-bootstrap/` | Bootstrap fixture for tests; may lag the root blueprint, gate-asserted to scan clean (`tests/examples_gate.rs`). |

When implementing a feature phase with a paired `phase-<N>.0-tests` change, remove the matching `#[cairn_planned(phase = <N>)]` attribute as the feature lands rather than rewriting those tests from scratch. The attribute is structured (proc-macro), not a comment; do not parse the `#[ignore]` reason string.

## Terminology

CAIRN spec is v0.8. Use `blueprint`/`.blueprint` (not `DSL`/`.dsl`) and `map`/`map.md` (not `ontology`/`index.md`) in all new prose, code identifiers, and spec drafts; the phase 2.6 rename is applied and archived (merge commit `3f15946`). `DSL`/`.dsl` string literals in `src/cli/mod.rs` and `src/blueprint/parser.rs` are intentional legacy-file detection; leave them.

Preserve these distinctions; the taxonomy encodes them deliberately, so do not propose flattening it:

- `reconciler` (pluggable interface), `scanner` (engine), `scan` (verb/CLI): three distinct concepts.
- `artefact`: typed-schema kernel primitive (umbrella kept; direct types are contract, decision, todo, research, review, source).
- `rationale tension`: advisory non-blocking finding class, distinct from `interface contradiction` (blocking).
- `change` / `changes/`: carries delta semantics (ADDED/MODIFIED/REMOVED/RENAMED); `proposal.md` lives inside it.
- `neighbourhood`: graph-theoretic query primitive.
- `provenance chain` / `authority chain`: spec §3 spine (see above).
- `interface hash`, `ghost`/`synced`/`orphaned`, `drift`, `divergence`, `verified`/`external`/`unverified`, `hinge`: preserve these distinctions.

## Project state and artefacts

For project status, outstanding work, or the reasoning behind a decision, query cairn directly: `cairn status` and `cairn context` to orient, `cairn change list` and `cairn frontier` for what is next, `cairn get` / `cairn neighbourhood` / `cairn decisions` for a node, `cairn lint --json` for structured findings. The graph is the source of truth, never markdown files, strongholds, or memory; anything under `docs/` or `archive/` is secondary context. The router's command reference lists every command.

`cairn scan --strict` is the verification gate (non-zero on Error or Warning), and a clean `cairn scan` (zero findings) is the target state: every new source file falls under a node `path` in `cairn.blueprint`, else extend a module or declare a new one. Record friction with `cairn feedback "<msg>"`.

When creating a decision, research finding, or source, follow docs/conventions.md section 10: files go flat under `meta/decisions/`, `meta/research/`, or `meta/sources/`, filename slug-only with the typed prefix only in the `id:` frontmatter (`dec.<slug>`, `res.<slug>`, `src.<slug>`; group via slug namespacing, never subfolders). Decisions require `id`, `nodes:`, `status`, `date` and chain to evidence via `informed_by:`; research requires `id`, `nodes:` and cites `sources:`; sources require `id`, `file:`, `verification:` and carry no `nodes:` field.

## Put decisions to the maintainer in-session

When a decision needs the maintainer's signature, put it to them in the session rather than leaving it to be discovered: the ruling in one or two sentences, your recommendation, what accepting changes, what rejecting costs, stated as a forced choice, never a hedge. `cairn pending` is the queue; the artefact stays the authority for detail. This holds in any in-harness session.

Which decisions still need that signature is governed by `dec.reviewer-panel-ratification`: a convergent binding ruling is accepted on adversarial panel receipts, and only its four pre-hoc classes (a genuinely balanced fork, maintainer-external stakes, mission or regime changes, reversal of a personally signed ruling) go to the maintainer before acceptance. Panel acceptances are put to the maintainer as outcome summaries, not requests; the veto stands open afterwards.

## UI and visual work: use the design system

Any UI change (the webui at `src/ui_assets/`, any landing or marketing page, any new surface) pulls from the canonical design system at `docs/design-system/`; reuse its classes and tokens before inventing. Tokens in `tokens.css` are authoritative: take every colour, size, and motion value from them, never hardcoded hex or rem, on every surface (`scripts/check-design-tokens.sh` gates the main CSS surfaces; the rule covers the rest). Font lanes are split per `dec.marketing-visual-world` (product vs marketing; faces and consumption patterns in `docs/design-system/fonts.css` and `README.md`). A new token or component lands with `tokens.css`/`components.css`, the live reference `index.html`, and the README in the same commit; the live reference is the source of truth for visual output. Voice rules for all user-facing copy, including the em-dash ban: `docs/agent/voice.md`.

## Guardrails

- Implement only what your tasks.md specifies, inside your change scope; prefer the simpler reading of an ambiguous task, checking `proposal.md` and `design.md` before guessing.
- All Rust code passes the gates in `scripts/pre-archive-rust-gates.sh` before a task is marked complete.
- Justify any `unsafe` code in the phase design document, and give every `#[allow(...)]` a `// Reason:` comment; without those pairings, neither is accepted.
- Never bypass hooks: `git commit --no-verify`, `git push --no-verify`, and the `SKIP=hookid` env var are forbidden. If a hook fails, fix the underlying issue.
- Treat archived phases under `archive/openspec/changes-archive/` as read-only historical record; correct the present in a new artefact instead of rewriting them.
- No em-dashes in any prose in this repository (docs, decisions, code comments, commit messages included); the pre-commit hook enforces this across all `*.md` files (`archive/` and `docs/research/` excepted). Replace with a period, colon, comma, or parenthesis.

See `docs/agent/principles.md` for the positive-form counterpart to these guardrails.

## Pre-submit review: mandatory

Before submitting any PR, run a self-review simplification pass (remove dead code, fix naming) followed by an adversarial review pass (catches bugs, logic errors, convention violations); in Claude Code these are `/reforge` then `/debate` (or `/palantir-debate`), in other harnesses run the equivalent read-only reviewer subagents in sequence. Fix anything surfaced before submitting. This applies to every PR in a stack; skip only for a single-line documentation change.

When asked for a `/debate`, or a sign-off question merits one, structure the response as three self-contained paragraphs: **For** (steel-man the strongest argument in favour), **Against** (steel-man the strongest counter-argument), **Verdict** (the decision, and why it outweighs the opposing view, ending on a forced decision line, not a hedge).

## Task tracking: native Todo artefacts are the front door

This repo's own development uses cairn's native Todo artefact (`dec.native-todos-first`). Add work with `cairn todo new <slug> --node <id>`; move status (`open`, `in_progress`, `done`, `blocked`) only through the sanctioned write verb `cairn todo set <slug> <status>` (`dec.cli-agent-workflow-consolidation`), never ad-hoc file edits. Inspect with `cairn todos <node>`.

## Developing cairn itself: the dev loop

To develop cairn itself, run the Cairn Dev Loop: the sole normative procedure is `cairn-dev` loop mode (`.claude/skills/cairn-dev/references/loop-mode.md`) plus exactly the required asset closure it declares; `/cairn-loop` is transport, and `docs/agent/cairn-dev-workflow.md` is descriptive, never normative. Loop mode runs only on explicit selection; the router never enters it from broad matching.
