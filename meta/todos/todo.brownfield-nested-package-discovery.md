---
node: cairn.brownfield
status: done
created: 2026-08-09
---

# Discovery misses workspace packages whose sources sit below the depth limit

Filed from the Arm A brownfield stress test over TrySita/AutoDocs
(`res.autodocs-arm-a-brownfield-run`, source `src.autodocs`).

## Evidence

`discover` in `src/brownfield/discovery.rs` records a directory only when it
holds `MIN_FILES` (3) source files **directly**, and `collect_source_files`
prunes recursion at `depth > MAX_DEPTH` (4), counting the repository root as
depth 0. Both together decide whether a package is seen at all.

On AutoDocs this splits two sibling pnpm workspace packages:

- `webview/apps/webapp` (depth 3) holds 4 loose `.ts`/`.tsx` files at its root,
  becomes a candidate, and its `path` then transitively owns the 85 TypeScript
  files beneath it.
- `webview/packages/shared` (depth 3) holds 1 loose file, its `src` (depth 4)
  holds 1, and its real source lives in `src/tools` (depth 5, 4 files) and
  `src/db/migrations` (depth 6). Every branch holding enough files sits past the
  prune, so the package produces zero candidates and its 10 tracked TypeScript
  files all surface as `CAIRN_RECONCILE_ORPHANED_FILE`.

10 of the run's 12 orphan findings trace to this one cause. Discovery quality
therefore keys on how many loose files happen to sit at a package root, not on
how much source the package contains.

## Scope

Decide the rule change, then implement it. Two candidates satisfy the Acceptance
below:

- Count source files recursively, within a bound, when deciding whether a
  directory qualifies.
- Or treat a directory holding a package manifest (`package.json`,
  `pyproject.toml`, `Cargo.toml`, `go.mod`) as a candidate root regardless of
  its direct file count.

Raising `MAX_DEPTH` alone does **not** qualify. Direct-file counting would still
apply, so AutoDocs would gain a leaf candidate at `webview/packages/shared/src/tools`
rather than a candidate for the package root, which is the opposite of what this
todo asks for and collides with the guard below.

Whichever is chosen, keep the existing guard against proposing a candidate per
nested leaf directory: the current thresholds exist to stop that.

## Acceptance

- A test over a nested-workspace fixture (a package whose sources sit two levels
  below its manifest) asserts the package yields a candidate whose `path` is the
  manifest-owning package root, and that its qualifying descendant directories
  are not additionally emitted as separate candidates.
- Re-running Arm A over AutoDocs reports fewer than 12
  `CAIRN_RECONCILE_ORPHANED_FILE` findings, with `webview/packages/shared`
  covered by a node whose path is `webview/packages/shared`.
