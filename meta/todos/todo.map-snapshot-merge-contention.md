---
node: cairn.kernel.scanner
status: done
created: 2026-07-16
related: [dec.persistent-map-snapshot]
---

# Reduce map.json merge contention between concurrent PRs

`dec.persistent-map-snapshot` commits `map.json` deliberately: deterministic
and timestamp-free, so its git diff shows real architectural drift on every
PR. Its revisit triggers cover file size and comparison needs, but not merge
contention. Observed live on 2026-07-16: PRs #361 and #362 both regenerated
`map.json`; after #361 merged, #362 went CONFLICTING and needed a manual
merge, rescan, and push. Every pair of concurrent PRs that adds a node,
artefact, or finding will pay this toll, which scales badly with contributor
count. This observation is a de facto new revisit trigger for
`dec.persistent-map-snapshot`.

## Task

Evaluate and implement the cheapest mitigation that preserves the decision's
reviewable-drift rationale:

1. Preferred: a custom git merge driver (`.gitattributes` entry plus a
   documented driver command) that resolves `map.json` conflicts by
   regenerating the snapshot with `cairn scan` on the merged tree. Conflict
   resolution becomes mechanical; the committed snapshot and its PR diff
   stay.
2. Fallback: stop committing `map.json` from PRs and have CI regenerate it
   on main after each merge. Removes contention entirely but weakens the
   in-PR drift diff, so it needs a decision amendment if chosen.

Untracking `map.json` outright reverses the decision's rationale and is out
of scope. Whichever option lands, record the outcome as a decision that
amends or reaffirms `dec.persistent-map-snapshot`, and document the setup
step (merge drivers are per-clone configuration) where contributors will
find it.

## Acceptance

Two branches that each add a distinct artefact and regenerated `map.json`
merge cleanly one after the other without a manual conflict-resolution
session, either via the merge driver or via CI regeneration. The chosen
mechanism is documented and its decision artefact links back to this todo
and to `dec.persistent-map-snapshot`.


## Resolution

Resolved 2026-07-16 with the preferred custom Git merge driver. The
`.gitattributes` entry registers `map.json merge=cairn-map`; the executable
`scripts/merge-map-json.sh` regenerates the snapshot with `cairn scan --strict`
and fails loudly when regeneration is unavailable. The one-time per-clone
configuration is documented in `README.md` and `docs/conventions.md`, and
`make install-hooks` applies it. Decision `dec.map-snapshot-merge-driver`
amends and reaffirms `dec.persistent-map-snapshot`; `map.json` remains tracked
and the CI-regeneration fallback was not needed.
