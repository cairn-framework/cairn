---
id: dec.map-snapshot-merge-driver
nodes:
  - cairn.kernel.scanner
status: accepted
date: 2026-07-16
informed_by: []
related:
  - dec.persistent-map-snapshot
---

# Resolve committed map snapshots with a merge driver

## Context

`todo.map-snapshot-merge-contention.md` records the observed contention, while
`dec.persistent-map-snapshot` deliberately commits deterministic, timestamp-free
`map.json` so every pull request shows architectural drift. That policy caused
live merge contention on 2026-07-16: PRs #361 and #362 both regenerated the
snapshot, and #362 required a manual merge, rescan, and push after #361 landed.

## Decision

AMEND and reaffirm `dec.persistent-map-snapshot`: keep `map.json` tracked and
reviewable, and register `map.json merge=cairn-map` in `.gitattributes`. The
`scripts/merge-map-json.sh` driver activates only for a plain `git merge`
(it requires the `GITHEAD_<sha>` environment variables, which Git sets before
invoking merge drivers during a real merge); it reconstructs the merged tree
in a temporary Git worktree, regenerates the snapshot there with
`cairn scan --strict` (preferring `cargo run --release` in a Cargo checkout,
falling back to an installed `cairn` if the local build is unavailable or
fails), and writes that result to the Git merge output. It fails loudly,
leaving a normal Git conflict, when there is no `GITHEAD_<sha>` environment
variable (rebase or cherry-pick; rare, and not the concurrent-PR contention
this decision targets, so those conflicts are resolved manually by
re-running `cairn scan`) or when neither scan path can regenerate the
snapshot. A failed regeneration therefore remains a normal Git conflict
rather than silently accepting an incorrect file.

The driver is per-clone configuration. Contributors run
`git config merge.cairn-map.driver 'scripts/merge-map-json.sh %O %A %B %P'`
and `git config merge.cairn-map.recursive binary`, or run
`make install-hooks`, which installs the hooks and this configuration. The
second config line stops Git from reusing this driver to compute a virtual
ancestor in a criss-cross merge (an internal merge step, unrelated to the
real one, where reusing a regenerating driver has no coherent meaning);
`binary` is Git's built-in driver, which never guesses and always reports a
conflict for that internal step.


## Limitations

GitHub's server-side mergeability check does NOT run custom merge drivers, so a concurrent PR will still show as CONFLICTING in the GitHub UI after its sibling merges. The value of this driver is that resolving the conflict locally becomes a single mechanical command rather than a manual editing session:
```sh
git fetch origin && git merge origin/main
# (the driver runs locally, regenerates map.json cleanly, and resolves the conflict)
git push
```
Nobody should expect GitHub's web UI to automatically resolve the conflict banners.
## Rejected alternative

Do not untrack `map.json`, because that would remove the in-PR drift diff that
motivated `dec.persistent-map-snapshot`. CI-only regeneration after merge is
also not chosen while the merge driver provides a local, mechanical resolution
without weakening reviewability.

## Consequences

Concurrent branches can each commit regenerated snapshots and merge without
manual JSON conflict editing. A clone must configure the driver once, and a
clone without a usable Cairn build receives the ordinary conflict for manual
resolution. The snapshot remains a committed derived measurement record, not a
second source of truth.
