---
id: dec.init-emits-agent-skills
nodes:
  - cairn.kernel.cli
status: accepted
date: 2026-06-26
---
# `cairn init` emits the cairn dev-loop agent skills

## Decision

`cairn init` now writes the curated cairn dev-loop skills into
`.claude/skills/` in the target repo, alongside the existing starter blueprint,
config, and `.cairn/AGENTS.md` guide. The emitted set is the loop on-ramp:
`cairn-dev` (entry point) plus its `references/`, and the `cairn-explore`,
`cairn-propose`, `cairn-apply`, and `cairn-archive` companions. The agent guide
(`.cairn/AGENTS.md`) gained a section pointing the agent at
`.claude/skills/cairn-dev` (cairn-48s).

## Single source of truth, no drift

The skills are compiled into the binary with `include_str!` from cairn's own
`.claude/skills/` tree, the same skills cairn develops itself with. There is no
second copy to maintain: the emitted pack and the skills in this repo are the
same bytes. The write reuses the existing idempotent scaffold loop, so it skips
any file already present and never clobbers a user's edits.

## Scope: the on-ramp pack, not the full triage

This delivers the deterministic, well-anchored half of distribution: emit the
vetted, git-tracked cairn-* loop skills so an external agent has an on-ramp to
the loop. It does not perform the managed-skill triage that decides the
broader install-pack membership (cairn-qgn), which refines what belongs in the
pack on top of this mechanism. `karpathy-guidelines` is a generic coding skill,
not a cairn-* skill, so it is not bundled.

An on-ramp is only useful if its commands are real. Bundling surfaced that the
v1 auto-generated skills had drifted from the CLI: `cairn-propose` named a
non-existent `cairn apply` command, `cairn-archive` taught a manual `mv` of the
change directory that bypasses the validated `cairn archive` command (which also
applies the blueprint delta), and `cairn-dev` pointed at stale `openspec/` paths
that `init --from-code` no longer writes. Those references were corrected in the
same change so the emitted pack is followable, not just present.

## Why init, not a separate subcommand

The bead's problem is that a fresh install gives an agent a guide but zero
on-ramp to the workflow that makes cairn useful. Emitting on `init` closes that
gap at the one moment a repo is scaffolded, with no extra step to remember. The
skills are inert files for non-Claude orchestrators, so the cost of emitting
them unconditionally is a few small markdown files, not behaviour change.
