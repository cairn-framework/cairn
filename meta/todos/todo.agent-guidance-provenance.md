---
node: cairn.kernel.cli
status: done
created: 2026-07-22
---

# Agent Guidance Provenance

## Priority

P0. Run first. The router migration cannot move loop authority until the
refining decision this unit proposes is accepted.

## Depends on

None.

## Scope

- Capture the Harness Engineering anthology as a Cairn source artefact pinned
  to commit 226c8d35fb6ea3ed55467753dba6dea2b5fd5778, listing the exact files
  relied on: `playbooks/improve-harness.md`, `docs/whole-job/README.md`,
  `docs/continuous-maintenance/README.md`,
  `docs/just-in-time-context/README.md`, `docs/authority/README.md`, and
  `sources/scripts/validate_manifest.py`.
- Distil the load-bearing mechanisms into a research artefact linked to
  `cairn.kernel.cli` and `cairn.kernel.query`: one accountable trajectory per
  job, just-in-time context routing, the continuous-maintenance loop contract
  (condition, drift signal, proof, authority, durable state, retirement),
  claim-boundary proof, and source-manifest integrity (one source owner per
  snapshot artefact path, leaf symlink and path-escape rejection, snapshot
  SHA-256 verification, and unowned-inventory failure).
- Propose the refining decision for the unified `cairn-dev` entry with an
  explicit loop mode, informed by that research, preserving every clause of
  `dec.loop-command-harness-model` and sanctioning only the later scoped
  migration of canonical loop authority.
- Separate external orchestration patterns, which stay in the harness, from
  Cairn-native authority per `dec.no-orchestrator`.

## Acceptance

- `meta/sources/harness-engineering.md` exists with the pinned commit and a
  verification field; `cairn scan` stays clean.
- `meta/research/harness-engineering.md` cites that source and links the
  relevant nodes.
- A proposed decision for the unified entry is authored and linked, ready for
  owner ratification. Acceptance sanctions a later migration; authority moves
  only when that cutover lands.
- `cairn lint` and `cairn scan` report zero new findings.

