# Cairn: Agent Orientation

Cairn is a graph-based architecture map for codebases. It models systems, containers, modules, and actors as nodes connected by dependency edges: a navigable structural graph. Each node has depth (code targets, contracts, artefacts like decisions/todos/research) and temporal history (changes, archive trail, decision lineage). The graph is the source of truth for what exists, how it connects, and why it's shaped that way.

## Your task context

Your change directory (`meta/changes/<change-id>/`) contains everything you need: `proposal.md` (why), `design.md` (how), `tasks.md` (what), and `specs/` (acceptance criteria). Work from these files. The quality gates in `scripts/pre-archive-rust-gates.sh` gives you the build/lint requirements.

## Where things live

| Path | What |
|---|---|
| `docs/conventions.md` | Rust code conventions (error codes, module size, state versioning, testing, docs). Authoritative; do not duplicate. |
| `docs/registries/` | `declared-items.md`, `error-codes.md`. Check when adding new public items or error codes to avoid collisions. |
| `archive/openspec/changes-archive/<other-phase>/specs/` | Other phases' acceptance criteria. Check only if your design.md references them. |
| `docs/spec.md` | Canonical Cairn spec. |
| `docs/design-system/` | Canonical design tokens, components, and live reference for any UI work. |

## Check if relevant, don't read by default

- **Conventions**: `docs/conventions.md` covers cross-cutting rules (error codes, naming, module limits). Check when making structural or naming decisions.
- **Registries**: `docs/registries/` covers declared items and error codes across all phases. Check when adding new public items or error codes to avoid collisions.
- **Specs from other phases**: `archive/openspec/changes-archive/<other-phase>/specs/` is only relevant if your design.md references another phase's requirements.

When implementing a feature phase, check `docs/conventions.md` for the test-first pre-phase convention. If a paired `phase-<N>.0-tests` change exists, remove the matching `#[cairn_planned(phase = <N>)]` attribute as the feature lands rather than rewriting those tests from scratch. The attribute is structured (proc-macro), not a comment; do not parse the `#[ignore]` reason string.

## Project state and artefacts

For project status, outstanding work, or the reasoning behind a decision, **query
cairn directly**. Do not infer state from markdown files, strongholds, or memory;
the graph is the source of truth.

```bash
cairn status              # project summary: nodes, findings, backlog. Start here.
cairn changes             # active change proposals.
cairn decisions <node>    # provenance chain for a node.
cairn research <node>     # research linked to a node.
cairn sources <node>      # external material a node cites.
```

If asked "what's next", start with `cairn status` and `cairn changes`, then your
issue tracker. Any file under `docs/` or `archive/` is secondary context, never
current state.

When **creating** a decision, research finding, or source, place it in `meta/` following
the convention in docs/conventions.md section 10 ("Artefact organization and provenance
links"):

- `meta/decisions/dec.<slug>.md` — requires `id`, `nodes:`, `status`, `date`.
  Chain to evidence via `informed_by: [res.X, src.Y]`.
- `meta/research/res.<slug>.md` — requires `id`, `nodes:`. Cite sources via
  `sources: [src.Z]`.
- `meta/sources/src.<slug>.md` — requires `id`, `file:`, `verification:`. No
  `nodes:` field; anchors transitively through citations.

Files are FLAT (no subfolders). Use slug namespacing for grouping
(`res.gas-city.analysis`, not `research/gas-city/analysis.md`).

## UI and visual work: use the design system

Any UI change (the webui at `src/ui_assets/`, any landing or marketing page, any new surface) pulls from the canonical design system at `docs/design-system/`. Do not re-invent styling.

- **Tokens are authoritative.** Colors, type, spacing, radius, shadow, and motion come from `docs/design-system/tokens.css`. Do not hardcode hex values or rem values in components, pages, or stylesheets.
- **Reuse components before inventing.** Classes defined in `docs/design-system/components.css` must be reused by class name before a new component is introduced. If something close already exists, extend it rather than parallel-building.
- **Font authority.** Source Serif 4 for headings and long-form copy. IBM Plex Mono for code and technical vocabulary. Both are wired up in `docs/design-system/fonts.css`.
- **When adding a new token or component**, update `docs/design-system/tokens.css` or `docs/design-system/components.css`, update the live reference at `docs/design-system/index.html`, and note the addition in `docs/design-system/README.md`. All four move in the same commit.
- **Live reference.** Open `docs/design-system/index.html` directly in a browser. It is the source of truth for visual output; if the page does not render as intended, the system is wrong, not the page.
- **Em-dashes are banned in user-facing copy.** Replace with a period, colon, comma, or parenthesis as context dictates. This applies to UI strings, marketing copy, and any prose that reaches a reader.

## Guardrails

- Implement only what your tasks.md specifies. Do not add features from other phases.
- Do not modify files outside your change scope unless your design.md explicitly requires it.
- If a task is ambiguous, prefer the simpler interpretation. Check `proposal.md` and `design.md` before guessing.
- All Rust code must pass the gates in `scripts/pre-archive-rust-gates.sh` `apply_prompt` before marking a task complete.
- No `unsafe` code unless your phase design document justifies it.
- No `#[allow(...)]` without a `// Reason:` comment.

## Task tracking: bd is this repo's tracker; native Todo is the artefact authority

This repo tracks work in **bd (beads)**, its optional Layer-1 storage backend (`dec.no-orchestrator` Layer 1; `dec.bd-upgrade-plan`: "this repo tracks work in beads"). That is a project workflow convention, the kind spec §5 says belongs in AGENTS.md, not a framework mandate.

Cairn's **native Todo artefact** (`docs/spec.md` §8.2, "Todo (authority)") remains the shipped artefact authority and is unchanged. bd does not replace it: bd issues carry a `cairn-node:<id>` label and surface per node through the read-only beads view (`src/state/backlog.rs`), so node-linked work appears in `cairn neighbourhood <node> --include-todos` without minting Todo files.

The two "Beads Issue Tracker" blocks below were generated by `bd setup`; their task-tracking Rules lines have been **reconciled in place** to match this section (bd is this repo's durable tracker, not a universal mandate over cairn's native Todo). They are now hand-maintained: do not blindly re-run `bd setup`, which would regenerate the universal-mandate wording ("do NOT use TodoWrite", "no markdown TODO lists"). If a tool regenerates them anyway, re-apply this reconciliation. This is `dec.native-task-state-and-agent-guidance` ruling 2's "pin-or-regenerate" fix.

<!-- BEGIN BEADS INTEGRATION v:1 profile:minimal hash:970c3bf2 -->
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

- Use `bd` for this repo's durable task tracking (its optional Layer-1 backend, per `dec.no-orchestrator`). Cairn's native Todo artefact (spec §8.2) remains the artefact authority and is not superseded; see "Task tracking" above. Ephemeral in-session phase tracking is fine and distinct from durable bd issues.
- Run `bd prime` for detailed command reference and session close protocol
- Use `bd remember` for persistent knowledge — do NOT use MEMORY.md files

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
   bd dolt push
   git push
   git status
   ```
5. **Hand off** - Summarize changes, validation, issue status, and any blocked sync/commit/push step

**Critical rules:**
- Explicit user or orchestrator instructions override this Beads block.
- Do not commit or push without clear authority from the active profile or the current user request.
- If a required sync or push is blocked, stop and report the exact command and error.
<!-- END BEADS INTEGRATION -->

<!-- BEGIN BEADS CODEX SETUP: generated by bd setup codex -->
## Beads Issue Tracker

Use Beads (`bd`) for durable task tracking in repositories that include it. Use the `beads` skill at `.agents/skills/beads/SKILL.md` (project install) or `~/.agents/skills/beads/SKILL.md` (global install) for Beads workflow guidance, then use the `bd` CLI for issue operations.

### Quick Reference

```bash
bd ready                # Find available work
bd show <id>            # View issue details
bd update <id> --claim  # Claim work
bd close <id>           # Complete work
bd prime                # Refresh Beads context
```

### Rules

- Use `bd` for this repo's durable task tracking; cairn's native Todo artefact (spec §8.2) is the artefact authority (see "Task tracking" above). Ephemeral in-session phase tracking is fine and distinct from durable bd issues.
- Run `bd prime` when Beads context is missing or stale. Codex 0.129.0+ can load Beads context automatically through native hooks; use `/hooks` to inspect or toggle them.
- Keep persistent project memory in Beads via `bd remember`; do not create ad hoc memory files.

**Architecture in one line:** issues live in a local Dolt DB; sync uses `refs/dolt/data` on your git remote; `.beads/issues.jsonl` is a passive export. See https://github.com/gastownhall/beads/blob/main/docs/SYNC_CONCEPTS.md for details and anti-patterns.
<!-- END BEADS CODEX SETUP -->
