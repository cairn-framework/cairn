---
id: res.autodocs-arm-a-brownfield-run
nodes:
  - cairn.brownfield
  - cairn.kernel.hooks
  - cairn.kernel.artefacts
sources: [src.autodocs]
method: primary
date: 2026-08-09
---

# Arm A: Cairn brownfield onboarding over the AutoDocs repository

The one-sided stress test mandated by `todo.autodocs-head-to-head` after
`dec.autodocs-head-to-head-arm-b` dropped Arm B. Arm B stays dropped for the
reason that decision records: AutoDocs supports neither polyglot nor nested
repositories, and its own repository is both.

## What was run

Target: `TrySita/AutoDocs` at `795ff04ddf6637cf044424f93c9fa807e08181cc`, the
exact commit `src.autodocs` pins. A fresh `git clone` landed on that commit with
no checkout needed.

Cairn: `cairn-framework` 0.9.0 at `603c22f6`, built from the loop worktree,
debug profile.

Commands run from a fresh clone. This is the measured procedure, not a
transcript of the six next actions `cairn init --from-code` prints:

```
git clone https://github.com/TrySita/AutoDocs.git
cairn init --from-code --apply
cairn scan
cairn hook all
```

## Repository shape

| Property | Value |
|---|---|
| Tracked files | 284 |
| Python files | 71 |
| TypeScript files | 61 `.ts` plus 46 `.tsx` |
| Layout | nested: `ingestion/` (Python), `webview/` (pnpm workspace, two packages) |

This is a medium repository, not a large one. It is a real polyglot nested
target, which is what the unit asked for, but see "What this does not settle"
below.

## Measurements

| Step | Wall time (debug build, across clean runs) |
|---|---|
| `cairn init --from-code --apply` | 0.43 s to 0.48 s |
| `cairn scan`, cold | 0.25 s to 0.26 s |
| `cairn scan`, warm | 0.07 s to 0.08 s |
| `cairn hook all` | 37 ms self-reported |

Single-run wall times vary a little between clones; the ranges are what was
observed, not a benchmark.

| Outcome | Value |
|---|---|
| Nodes discovered | 12 (10 Python, 2 TypeScript) |
| Inferred edges | 4 |
| Stub contracts written | 12, one per node |
| Findings after first scan | 13, all Info |
| `CAIRN_RECONCILE_ORPHANED_FILE` | 12 |
| `CAIRN_ORDER_CYCLE` | 1 |
| `cairn scan --strict` | exit 0 |
| `cairn hook all` | exit 1 (see defect 1) |

The run is reproducible: the whole sequence was executed three times from
separate clean clones of the pinned commit and produced the identical outcome
table every time, including the failing `cairn hook all`.

## Defects found

### 1. The first `cairn hook all` after onboarding errors out

Reproduced from a clean clone. Immediately after
`cairn init --from-code --apply`, `cairn hook all` exits 1 with
`Error: CAIRN_HOOK_AFFECTS_SUBSET cannot read or parse the candidate blueprint
while checking ratification evidence`.

The trigger is an untracked or unstaged blueprint, not an uncommitted one.
Measured on three states of the same clone:

| State of `cairn.blueprint` | `cairn hook all` |
|---|---|
| untracked, nothing staged | exit 1, Error |
| staged, not committed | exit 0 |
| committed | exit 0 |

Staging alone clears it. The ratification hook reads the candidate blueprint
from the git candidate tree, the index under the default `RatificationMode`
and `HEAD` in CI (`src/hooks/ratification/git.rs:107-119`), and
`candidate_local_decisions` turns a `None` from that read into a hard Error
(`src/hooks/ratification.rs:135-143`). A freshly onboarded repository has
`cairn.blueprint` untracked, so the candidate read has nothing to return.

This is the adopter's first gate result. The next-steps ladder that
`cairn init --from-code` prints ends with "6. Check the commit gate:
`cairn hook all`" and never says to stage first, so following the printed
sequence in order meets a hard Error. Filed as
`todo.brownfield-first-hook-blueprint-unstaged` (node `cairn.kernel.hooks`).

### 2. Nested workspace packages are invisible to discovery

`discover` records a directory only when it holds three source files
*directly*, and prunes recursion below depth 4, counting the repository root as
depth 0. On AutoDocs this splits two sibling pnpm workspace packages by an
accident of file placement:

- `webview/apps/webapp` (depth 3) keeps 4 loose files at its root, becomes a
  node, and transitively owns the 85 TypeScript files beneath it.
- `webview/packages/shared` (depth 3) keeps 1 loose file at its root and 1 in
  `src` (depth 4), with its real source at `src/tools` (depth 5, 4 files) and
  `src/db/migrations` (depth 6). Every branch holding enough files is pruned, so
  the package yields zero candidates and all 10 of its tracked TypeScript files
  surface as orphans.

10 of the 12 orphan findings trace to this single cause. Discovery quality keys
on loose-file placement, not on how much source a package holds. Filed as
`todo.brownfield-nested-package-discovery`.

### 3. Manifest-derived `node_modules` ignore entries are redundant

`cairn init --from-code --apply` wrote 15 `*/node_modules` entries into
`cairn.config.yaml`, one per directory holding a `package.json`, without
checking existence. On this clone none of the 15 directories exist.

They are also redundant with ignores cairn already applies unconditionally.
`node_modules` is in the scanner's built-in ignore list
(`src/scanner/config/mod.rs:155-159`) and is hard-coded in discovery's
`is_ignored_dir` (`src/brownfield/discovery.rs:188-194`), so a path-specific
entry adds nothing whether or not the directory exists.

An attempt to demonstrate the redundancy empirically did not produce usable
evidence and is not relied on here. Injecting a `node_modules` tree into the
clone left the finding count unchanged, but so did an identically placed
control directory under a non-ignored name, so the run cannot attribute the
exclusion to the ignore machinery rather than to what the reconciler
enumerates. Establishing that behaviourally needs a fixture built for it. The
redundancy claim above rests on the two source facts, not on that attempt.

Filed as `todo.brownfield-redundant-node-modules-ignores`.

## A fourth defect, surfaced by reconciling rather than by the target

Correcting the item 7 claim required a proposed decision superseding a
maintainer-ratified one, which is exactly the shape `cairn-loop-reconcile`
section 4 prescribes. It cannot be expressed. A `supersedes:` pointer whose
target is still `accepted` raises `CAIRN_DECISION_SUPERSEDES_STATUS` and fails
`cairn scan --strict`, while demoting the target would self-ratify an amendment
the loop may not make. The link was therefore deferred to acceptance and the
intent carried in prose. Filed as `todo.decision-proposed-supersession-shape`
(node `cairn.kernel.artefacts`).

## Confirmations

### 1. Polyglot onboarding works, unconfigured

Both languages were parsed and both produced nodes in one pass with no language
configuration and no per-language flags. This is the capability AutoDocs' own
README lists as unsupported, so the asymmetry `dec.autodocs-head-to-head-arm-b`
relied on is real in both directions: Cairn ingests the target, and the target
cannot ingest itself.

The coverage is uneven rather than absent. 10 of the 12 nodes are Python. Of the
two TypeScript nodes, only `webview.apps.webapp` is a workspace package;
`ingestion.tests.test-ast-parsing.test-files` is a directory of 4 TypeScript
parser fixtures inside the Python test tree. So one of AutoDocs' two real
TypeScript packages is mapped and the other is not. That unevenness is defect 2,
not a language-support gap.

### 2. Inferred-edge conservatism holds on a real repository

4 edges were emitted, each carrying `@inferred` provenance and its observation
count in the description. No all-pairs sibling guessing appeared, which is the
failure mode `discovery.rs:82-85` says the design exists to prevent.

The one cycle reported is genuine, not an artefact:
`ingestion/src/ast_parsing/*` imports `.utils.*` (9 observations) and
`ingestion/src/ast_parsing/utils/*` imports `..constants` (2 observations). It
is a child-imports-parent package cycle, which is ordinary Python, and it landed
as advisory Info rather than a blocking Error, exactly as
`dec.brownfield-discovery-cycle-severity` specifies.

### 3. The strict scan is green on an unrefined map

`cairn scan --strict` exited 0 on the first scan, with all 13 findings at Info.
The strict gate an adopter would wire into CI therefore passes on a map nobody
has curated yet. This is narrower than "onboarding is green": the printed ladder
still ends in a failing `cairn hook all`, which is defect 1.

## What this does not settle

`res.codeatlas-analysis` item 7 defers JSON-surface token budgets "until a large
brownfield dogfood repo exists to measure against". AutoDocs at 284 tracked
files is medium, so it is not the large target item 7 asked for. Note also that
nothing here measures item 7's actual quantity: no JSON output token counts were
taken, and scan latency is not a proxy for them. This run supplies a first
real-repository baseline and a repeatable procedure. That deferral stands.

No external contact of any kind was made. Everything here is local compute over
a public Apache 2.0 clone.
