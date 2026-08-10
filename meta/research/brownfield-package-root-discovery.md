---
id: res.brownfield-package-root-discovery
nodes:
  - cairn.brownfield
sources: [src.autodocs]
method: primary
date: 2026-08-10
---

# Package roots as discovery candidates, measured on AutoDocs

`todo.brownfield-nested-package-discovery` offered two rules that would let
discovery see a workspace package whose sources sit below the depth prune, and
ruled out a third. This records which was taken, why, and what the AutoDocs
rerun measured.

## Options weighed

1. **Count source files recursively, within a bound, per directory.** Fixes the
   count but not the anchor. The directory that wins is still whichever one the
   bound happens to reach, so a package whose sources sit two levels down still
   maps at `src/tools` rather than at the package root. It also needs a second
   guard to stop every ancestor of a dense subtree qualifying at once.
2. **Treat a directory holding a package manifest as a candidate root.** Taken.
   The manifest is the boundary the ecosystem already declares, so the anchor is
   read rather than guessed, and one package yields one candidate whatever the
   shape of its subtree.
3. Raising `MAX_DEPTH` alone. Ruled out by the todo, and correctly: direct-file
   counting would still apply, so AutoDocs would gain a leaf candidate at
   `webview/packages/shared/src/tools` and no candidate for the package root.

Option 2 needs two supporting rules that the todo did not name, both surfaced by
building it:

- **The depth budget restarts at a package root.** Without it the manifest is
  seen but its sources are not: `webview/packages/shared/src/tools` sits at
  depth 5 from the repository root and is pruned before any file is recorded.
  Bounding depth per package rather than per repository keeps the traversal
  bounded while making a package's own subtree reachable. It needs a second
  bound to stay honest: a repository carrying a manifest in every directory
  would restart the budget at every level, so an absolute ceiling on depth
  below the repository root sits above the per-package budget.
- **The innermost package wins.** A pnpm workspace root carries a manifest too.
  Left alone it would own every file under it and swallow the two packages the
  todo asks to separate, so a package root that encloses another package root is
  dropped in favour of the packages inside it.
- **Nothing inside a package-root candidate is proposed separately, and a
  manifest directory never returns through the direct-count rule.** Dropping
  the enclosing root is not enough on its own: pre-submit review found that a
  workspace root with three loose files of its own came back through the
  direct-count pass and restored the nesting the drop exists to prevent.

The same review proposed going further and rejecting any directory that merely
contains a package. That was built and measured, and it is wrong: AutoDocs went
from 2 orphan findings to 20, because dense directories such as
`ingestion/tests` sit above the TypeScript fixture packages and lost their own
candidates. The invariant it cited,
`dec.brownfield-discovery-cycle-severity` clause 2, rules that package roots
and subpackages stay flat sibling Modules, which governs emitted shape and not
path containment. Path containment between candidates therefore stays allowed.

The narrower rule still has a cost. A source file directly under a dropped
workspace root is claimed by nothing and reconciles as an orphan. That is
preferred to a workspace node owning its packages' files, which is the defect
being removed, but it means "accounts for every source file below it" holds of
surviving package roots only.

Qualification for a package root is ownership of at least one source file, not
`MIN_FILES`: the todo defines this option as candidacy "regardless of its direct
file count", and the guard against a candidate per nested leaf is the ownership
rule, not the threshold. `MIN_FILES` still gates every directory that no package
claims.

## Measurement

Arm A rerun, same procedure and same pinned target as
`res.autodocs-arm-a-brownfield-run`: a fresh clone of `TrySita/AutoDocs` at
`795ff04ddf6637cf044424f93c9fa807e08181cc`, then `cairn init --from-code
--apply` and `cairn scan`. Cairn built from the loop worktree, debug profile.

| Outcome | Before | After |
|---|---|---|
| Nodes discovered | 12 | 20 |
| `CAIRN_RECONCILE_ORPHANED_FILE` | 12 | 2 |
| `CAIRN_ORDER_CYCLE` | 1 | 1 |
| Total findings | 13 | 3 |

The before column is the outcome table in `res.autodocs-arm-a-brownfield-run`,
measured three times from separate clean clones of the same commit.

`webview/packages/shared` is now a node whose `path` is
`./webview/packages/shared`, alongside `webview/apps/webapp`, which is what
defect 2 of the Arm A run asked for. The workspace root `webview` carries a
manifest but is not proposed, because the two packages inside it claim their own
files first.

The two orphans that remain are `ingestion/src/dag_builder/__init__.py` and
`netx.py`: a two-file directory inside a repository whose Python tree declares
no per-package manifest, so it is still governed by `MIN_FILES` and still below
it. That is the threshold working as designed, not the defect this unit fixed.

## What this does not settle

Python granularity survived by accident of layout rather than by design.
AutoDocs declares no manifest under `ingestion/`, so its ten Python modules
still map per directory. A Python repository that does declare one per package
would map that package as a single node instead. Whether that coarsening is
right is a separate question about what a discovered node should mean, and
nothing here measures it.

Node count rose from 12 to 20 partly because six of the new nodes are the
TypeScript workspace fixtures under `ingestion/tests/typescript-repos/`, each of
which carries its own `package.json`. They are test data, not product modules.
Discovery has no signal for that distinction and this unit did not add one.

`.tsx` remains outside `SOURCE_EXTS`, so AutoDocs' 46 `.tsx` files count toward
no candidate. That is untouched here and unmeasured.
