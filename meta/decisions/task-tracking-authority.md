---
id: dec.task-tracking-authority
nodes:
  - cairn.root
status: accepted
date: 2026-07-28
informed_by:
  - res.decision-accumulation-cairn-root
  - res.task-front-door
  - res.github-todo-sync
  - res.gas-city-cairn-integration
  - res.native-task-state-gap
supersedes:
  - dec.native-todos-first
  - dec.native-task-state-and-agent-guidance
  - dec.beads-task-layer
  - dec.bd-upgrade-plan
  - dec.bead-github-sync
  - dec.github-todo-sync-projector
  - dec.github-todo-issue-body-fidelity
related:
  - dec.cli-agent-workflow-consolidation
  - dec.change-format-only
revisit_triggers:
  - "a maintainer wants node-linked beads to become a first-class Todo artefact source, which requires a ratified amendment to the Todo schema first"
  - "bd drops or changes the passive `.beads/issues.jsonl` export contract or the `cairn-node:<id>` label convention, or changes `bd github` push/pull/sync"
  - "a concrete requirement needs a bd 1.0.5+ feature such as server-validated custom issue types, or the repository adopts a Dolt remote with `refs/dolt/*` on origin"
  - "a maintainer sanctions a one-way bead-to-GitHub projection, or accepts GitHub as a canonical or bidirectional store for either beads or Todos"
  - "a typed Todo relationship model is accepted and the projector is asked to emit cross-issue links"
  - "GitHub issue body-size limits truncate full Todo bodies in practice"
  - "unmapped-issue triage becomes a recurring bottleneck"
  - "a storage-backend decision ratifies the filesystem default with bd optional, after which this decision relates it and the agent guidance cites it"
  - "a source-symbol code-map extension is proposed, indexing enum variants, struct fields, function signatures, call sites, file, span, and kind so cairn can answer source-structure questions deterministically"
  - "`bd setup` regenerates and overwrites reconciled `AGENTS.md` blocks, requiring a pin-or-regenerate ruling"
---
# Task tracking authority: native Todo files are canonical, beads and GitHub are derived

## Context

`cairn.root` carried seven accepted decisions about where work items live and
who may write them. They were taken in four successive settings: beads as this
repository's tracker, beads plus a possible GitHub mirror, native Todo
artefacts taking primacy, and native Todos projected one way to GitHub issues.
Each later decision restated parts of the earlier ones to stay coherent, so the
lineage now reads as seven authorities where it expresses one contract.

`res.decision-accumulation-cairn-root` holds the measurement and the precedent.
This decision consolidates the lineage. It changes no behaviour, reopens
nothing, and retires no capability that is still in use.

## Decision

### 1. Native Todo files are the canonical store

This repository tracks its work in `meta/todos/todo.<slug>.md`, the same
mechanism a fresh `cairn init` gives an adopting repository, not in beads. A
Todo is markdown with `node:`, `status:`, and `created:` frontmatter. The files
are the source of truth; every other surface named below is derived from them.

`AGENTS.md` names native Todos as this repository's tracker and beads as an
optional integration for other projects. It does not instruct contributors to
use `bd` for new work here.

### 2. cairn scaffolds artefacts; it does not run task workflow

`cairn todo new <slug> --node <id>` is artefact scaffolding under
`dec.change-format-only`. It writes `node:`, `status: open`, and `created:`
frontmatter plus a slug-derived H1, and no workflow metadata beyond that:
`status: open` is the binding default, and cairn ships no claim verb and no
close verb. Work-item creation, claiming, and sequencing stay outside cairn.

Status transitions are a surgical frontmatter write, not free-form editing;
`dec.cli-agent-workflow-consolidation` owns the sanctioned write surface
(`cairn todo set <slug> <status>`) and supersedes the plain-file-edit wording
this clause replaced. Its `refines: [dec.native-todos-first]` therefore lands on
this clause.

### 3. Selection prefers open native Todos

`cairn next` and `cairn brief` prefer open native Todos, sorted by `created`
then filename, and consult the beads backlog only as a fallback when
`.beads/issues.jsonl` exists and no native Todo is open.

### 4. Beads remain a conditional, read-only, derived view

The beads integration is supported and unchanged, and it is subordinate:

1. It is a thin read-only projection over the `.beads/issues.jsonl` reader,
   filtered by the `cairn-node:<id>` label that `backlog.rs::linked_node()`
   parses, exposed through `query_api` as `cairn backlog <node>` and the
   per-node webui inspector view.
2. It never mints a Todo. It does not source a Todo body, `created` date, or
   `satisfies` clause, does not write `meta/todos/`, and does not redefine the
   Todo type. Beads becoming a genuine Todo source requires a maintainer
   ratified amendment to the Todo schema first.
3. Bead status is a navigational claim, never a reconciled fact. Nothing gates
   on it. Coverage enforcement stays keyed to node reconciliation state.
4. Bead `id`, `title`, `status`, and `priority` render verbatim; they are not
   mapped into `TodoStatus`. The recorded future-only mapping, if a mapping is
   ever needed, is `open` to `Open`, `in_progress` to `InProgress`, `closed` to
   `Done`, `blocked` to `Blocked`, and `deferred` to `Blocked`, there being no
   `Deferred` variant.
5. A bead without a `cairn-node:<id>` label is unlinked, not erroneous. A label
   resolving to an unknown or deleted node is an informational, non-blocking
   orphan warning.

### 5. Beads storage and upgrade rules, where beads are run

Conditional operational guidance, binding wherever the optional integration is
used, including here:

1. Keep jsonl-in-git where the export is tracked: Dolt as the local storage
   engine, `.beads/issues.jsonl` as the human-diffable, upsert-only
   cross-machine projection. Do not adopt a Dolt remote. This repository is the
   standing exception: `dec.gap-cairn-state-should-beads-jsonl-exports-stay-git-tracked-now`
   resolved its exports untracked as of 2026-07-11, and `.beads/.gitignore`
   still ignores `*.jsonl`. That ruling stands and is not reopened here.
2. Pin `export.auto: true` and `export.git-add: true` in `.beads/config.yaml`
   so a later upgrade cannot silently disable jsonl refresh and staging.
3. Stay on bd 1.0.4 until a revisit trigger fires. `bd github` and `--defer`
   already exist there. Use the `spike` label rather than `types.custom`;
   revisit promoting it only at 1.0.5+.
4. Never run `bd upgrade` blindly. First commit and push bead work and confirm
   the two export pins; then upgrade, run `bd doctor`, resolve migration
   content skew, make a trivial bead edit, verify `.beads/issues.jsonl`
   refreshes and is staged, and, where the export is tracked, commit it. A
   staged export is not propagated cross-machine.
5. If `refs/dolt/*` ever exists, every clone reaches identical state by
   `bd dolt push` and `pull` before anyone upgrades; one designated migrator
   upgrades with `BD_ALLOW_REMOTE_MIGRATE=1` and pushes, and every other clone
   then upgrades and pulls. Unsynchronised clones never independently cross
   migrations 0040-0042 / 0050.

### 6. GitHub is never canonical, for beads or for Todos

GitHub issues are not a second source of truth. Cairn owns no GitHub
coordination: no cairn command, hook, reconciler, or binary call reaches
GitHub, and no ForgeDock-style `workflow:*` / `bead:*` label scheme or
HTML-comment annotation layer is built, because it duplicates `bd github`.

No bead mirror is adopted. Bead-GitHub synchronisation stays deferred, and
adopting even the one-way shape described below requires a future superseding
decision; a canonical or bidirectional store is refused outright.

If a maintainer later sanctions one, its shape is already fixed: opt-in,
one-way, and read-only, implemented through `bd github push` (or `bd github
sync` constrained to push) invoked manually or from an orchestrator pack, using
`bd github`'s native bead-id to issue-number mapping. No invented
`bead-id:cairn-xxx` label, no hand-stored issue numbers, and no status or
priority mapping owned by cairn.

### 7. Native Todos project one way to GitHub issues

`meta/todos/*.md` is canonical and the issue set is a derived, regenerable
view:

1. `scripts/sync-github-todos.sh`, run by CI on pushes to main, is the only
   writer. It upserts one issue per Todo artefact, keyed by a
   stable marker line in the issue body.
2. The projected body is assembled in exactly this order: the marker
   `cairn-todo: todo.<slug>`; a deterministic `node:`, `status:`, `artefact:`
   header plus one single-line one-way note; then the complete post-frontmatter
   Todo markdown, H1 and every later section unchanged. The marker stays first
   because it is the projector's identity key. There is no multi-line
   disclaimer paragraph.
3. Rebody if and only if the newly rendered body differs from the current issue
   body or its stored hash. Body-only edits are caught; nothing rebodies every
   run; two consecutive runs without a file change perform no `issue edit`.
4. A Todo reaching `done` closes its mirrored issue.
5. Issues filed directly on GitHub are never auto-imported. They are labelled
   `cairn-todo-unmapped` for human triage, which either closes the issue or
   mints a native Todo by hand.
6. Nothing is read back from GitHub, ever.
7. `tests/sync_github_todos.rs` MUST assert the full create-body payload, a
   body-only edit producing `issue edit --body`, and a no-file-change second
   run producing no `issue edit`. None of clauses 7.2, 7.3, or this one is
   implemented yet: the script still writes the stub body and rebodies on
   status or node only. `todo.github-todo-full-issue-body` is the pending
   implementation.
8. The projector emits no cross-artefact relationship links, subtask graph, or
   dependency graph. Relationship and subtask projection stay blocked until a
   typed relationship model is decided and implemented.

### 8. Query cairn before grepping

For questions such as what state a node is in or which decisions affect it, use
`cairn rationale <node>`, `cairn context`, `cairn scan`, and `cairn lint`
before reading source files. `AGENTS.md` carries this rule.

### 9. What stays open

One direction from the superseded set is preserved as open, not resolved here.
The conflation between an intentionally unbuilt plan node and unexpected drift
is still undecided, and the preconditions on deciding it are still binding: the
dev plan resolves Path A against Path B before implementation begins, and
adversarial review stress-tests both against the two-chain model. Path A adds
`NodeState::Planned` derived from an explicit blueprint marker plus an absent
path, keeping absent-without-marker as `Ghost`, and amends the node-state model
with a fourth state. Path B keeps three variants, uses finding data already on
the node, requires no spec change, and distinguishes the cases visually: a ghost
with no findings renders calm, solid-outlined, and still, a ghost with drift
findings renders alarmed, dashed, and breathing. Either path updates
`src/ui/serialise.rs`, `src/cli/export/json.rs`,
`src/query_api/handlers/project.rs`, and the webui legend and node rendering.
`res.native-task-state-gap` holds the evidence.

## Rationale

Consolidation was chosen over leaving the lineage in place because the seven
decisions no longer disagree about anything: three of them describe an optional
integration that is now subordinate, two describe one projector and its
refinement, and two describe the same primacy ruling at proposal and
implementation stage. A reader asking "where do work items live" had to read
all seven and derive the ordering.

The alternative, superseding only the genuinely historical beads decisions, was
rejected: it would have marked a supported integration as retired, which is
false. Beads are demoted, not removed, and clauses 4 and 5 carry their rules
forward verbatim in substance.

## Consequences

- The seven named decisions are `superseded`. They drop out of default
  payloads, so every still-live `revisit_trigger` they carried is restated in
  this decision's header, including the unmapped-issue triage bottleneck, the
  filesystem-default storage ratification, the one-way bead projection, the
  export-contract change, and the source-symbol code-map extension. They keep their provenance and still
  count as provenance coverage for `cairn.root`.
- `dec.cli-agent-workflow-consolidation` records `refines: [dec.native-todos-first]`.
  That reference stays valid: it names a decision this one supersedes, and the
  refined rule now lives in clause 2.
- Prose that cites `dec.native-todos-first` (`AGENTS.md`, `docs/conventions.md`,
  `CHANGELOG.md`, several todos) continues to resolve. Following the precedent
  set by `dec.todo-write-surface` under the CLI consolidation, those citations
  are not rewritten; the id still names the decision that made the rule.
- Completed transitions are historical and are not restated as obligations: the
  migration of beads `cairn-380`, `cairn-m99`, `cairn-omf`, and `cairn-jj4` to
  `meta/todos/`, the removal of the `AGENTS.md` bd mandate, the generated-stub
  issue body, and the status-or-node-only rebody rule.
