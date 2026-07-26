---
node: cairn.kernel.scanner
status: open
created: 2026-07-26
---

# Map Snapshot Freshness Gate

## Problem

`map.json` is a committed, deterministic snapshot (`dec.persistent-map-snapshot`)
that `cairn scan` regenerates, but no gate asserts the committed copy matches
what a scan produces. PR #476 merged with a stale snapshot and every check
passed. Nothing in the pipeline could have caught it: `.github/workflows/ci.yml`
runs the Rust checks and validates the prek configuration,
`.github/workflows/dogfood.yml` runs `scripts/dogfood.sh`, which calls only
`cairn lint` and `cairn hook all`, and neither compares regenerated output.
`cairn scan --strict`, the one command that would notice, is not run by CI at
all, and it rewrites the file rather than comparing it, so it could not serve as
the gate unchanged. The only scripted `scan --strict` invocation in the
repository is `scripts/merge-map-json.sh`, which regenerates the snapshot while
resolving a merge conflict and is not a freshness check. The staleness surfaced
only because a later loop preflight found an unexplained dirty file in a parked
worktree.

That is the wrong detector. A committed derived artefact with no freshness check
drifts silently and is then repaired by whichever unrelated iteration happens to
notice, which is how `todo.map-snapshot-resync` came to exist.

## Scope

- Add a deterministic check that fails when the committed `map.json` differs
  from the snapshot a scan regenerates, so the drift blocks a commit or a PR
  rather than a later preflight.
- Decide where it belongs: a `scripts/check-*.sh` project-health gate in CI, a
  pre-commit hook step, or a `cairn scan` mode that compares instead of writes.
  Prefer the surface that fails on the authoring machine, since a CI-only gate
  leaves the contributor with a red build and no local reproduction.
- The check must not itself write `map.json`, or it defeats its own purpose.
- Confirm the merge driver in `scripts/merge-map-json.sh` still composes with
  whatever surface is chosen; that driver already regenerates the snapshot on a
  merge conflict.

## Acceptance

- Committing or pushing a source change that shifts a symbol range without the
  regenerated snapshot fails the gate, with a message naming the command that
  fixes it.
- A tree whose `map.json` matches the regenerated snapshot passes.
- Reverting the `map.json` half of the `todo.map-snapshot-resync` commit and
  running the gate reproduces the failure, which is the regression evidence.
