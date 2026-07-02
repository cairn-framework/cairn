---
id: dec.change-format-only
nodes:
  - cairn.kernel.changes
status: accepted
date: 2026-07-02
informed_by: [res.vision-refactor-audit]
related: [dec.beads-task-layer, dec.no-orchestrator]
revisit_triggers:
  - "grep for StateBackend/storage_backend/state_backend surfaces a production caller beyond change.rs and tests that was missed in the 2026-07-02 audit"
---

# Change system trimmed to format-only: scheduling is workflow, not cairn

## Context

`res.vision-refactor-audit` finding 4: `StateBackend`'s `storage_backend()`
factory is called only from `src/state/tests.rs`, confirmed production-dead.
The methods that *are* live are `create_change_epic`, `create_task_beads`,
`list_child_tasks`, and `claim_change` on `BeadsStateBackend`
(`src/state/beads.rs:202,225,265,301`), wired into `src/cli/commands/change.rs`.
These create, seed, and claim work items in beads on behalf of `cairn change
new`/`cairn change apply`. This is separate from and does not touch the
read-only backlog view `dec.beads-task-layer` already ratified
(`crate::state::backlog`, `cairn backlog <node>`): that view stays.

## Decision

**Criterion:** validating or applying declared state is cairn's job; creating,
claiming, or sequencing work items is workflow, and cairn does not do
workflow (`dec.no-orchestrator`).

Applying the criterion: delete `create_change_epic`, `create_task_beads`,
`list_child_tasks`, and `claim_change` from `BeadsStateBackend`, their call
sites in `src/cli/commands/change.rs`, and the `state_backend == "beads"`
config guard that selected them. Reduce `src/state/mod.rs` to a thin shell
around `pub(crate) mod backlog;` (the read-only view stays byte-identical, its
`crate::state::backlog::` import path is unchanged for its 8+ callsites);
delete `StateBackend`, `StateRecord`, `FilesystemStateBackend`,
`storage_backend()`, and `src/state/beads.rs` in full. `cairn change new`
keeps scaffolding `proposal.md`/`design.md`/`tasks.md`/`specs/` (a format
concern) but stops seeding bead task lines from `tasks.md`. `cairn change
tasks` and `cairn change apply` are deleted outright: their entire bodies
are `list_child_tasks`/`claim_change` respectively, with no other behaviour
to preserve. Delta/artefact application is a distinct, untouched command:
`cairn archive <change-id>` (`crate::changes::archive`), not `cairn change
apply`. `cairn accept` also stays: it validates suggested-edges triage and
runs gates, which is gating, not scheduling.

## Rationale

Keeping dead machinery (`StateBackend`) alive alongside workflow logic
(`create_change_epic`/`claim_change`) that contradicts `dec.no-orchestrator`
sends two conflicting signals to future contributors: that cairn tracks task
state internally (it doesn't; bd and beads do, per `dec.beads-task-layer`),
and that the change system schedules work (it applies declared deltas, it
does not sequence them). Removing both makes the change system's actual scope
match its name: change *format*, validation, and apply/archive.

## Consequences

- `cairn.config.yaml`'s `state_backend` key is removed, not deprecated in
  place (accept-and-ignore is prohibited); its docs mentions are updated in
  the same commit.
- Zero `crate::state::backlog::` callsites change; the read-only per-node task
  view (`dec.beads-task-layer`) is fully preserved.
- If `cairn.blueprint` declares a paths entry that becomes empty after
  `src/state/beads.rs`'s deletion, the node's `paths` are updated in the same
  commit so scan stays clean.
