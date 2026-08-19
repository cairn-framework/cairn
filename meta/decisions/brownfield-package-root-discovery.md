---
id: dec.brownfield-package-root-discovery
nodes:
  - cairn.brownfield
status: proposed
ratification: local
date: 2026-08-10
informed_by:
  - res.brownfield-package-root-discovery
related:
  - dec.brownfield-discovery-cycle-severity
  - dec.autodocs-head-to-head-arm-b
affects:
  - src/brownfield/discovery.rs
  - src/brownfield/walk.rs
  - src/brownfield/mod.rs
  - meta/contracts/brownfield.md
  - meta/decisions/brownfield-package-root-discovery.md
  - meta/research/brownfield-package-root-discovery.md
  - meta/todos/todo.brownfield-nested-package-discovery.md
revisit_triggers:
  - "a discovered package node is measured to be too coarse to review, so a package needs splitting into several candidates"
  - "a manifest filename outside package.json, pyproject.toml, Cargo.toml, or go.mod is needed to anchor a real adopter's packages"
---

# Brownfield discovery anchors candidates on package roots

## Context

Discovery recorded a directory only when it held three source files directly,
and pruned traversal four levels below the repository root. Both rules measure
placement, not content, so which package becomes a node depended on how many
loose files happened to sit at its root. On AutoDocs
(`res.autodocs-arm-a-brownfield-run`, defect 2) that split two sibling pnpm
workspace packages: `webview/apps/webapp` mapped and
`webview/packages/shared` produced nothing, leaving 10 of the run's 12 orphan
findings tracing to one cause.

`todo.brownfield-nested-package-discovery` named two admissible rules and ruled
out a third. `res.brownfield-package-root-discovery` weighs them.

## Decision

A directory holding a package manifest (`package.json`, `pyproject.toml`,
`Cargo.toml`, `go.mod`) is a discovery candidate root, and accounts for every
source file below it that no nearer package root claims. Four rules follow from
that anchor: the depth budget restarts at a package root rather than running
from the repository root, under an absolute traversal ceiling so manifests
nested all the way down cannot make the walk unbounded; where package roots
nest, the innermost wins and the enclosing one is dropped; nothing inside a
package-root candidate is proposed separately, and a manifest directory never
returns through the direct-count rule; and a package root qualifies on owning
at least one source file, while `MIN_FILES` continues to gate every directory
that no package claims.

Path containment between candidates is not forbidden. A dense directory that
merely contains a package keeps its own candidate:
`dec.brownfield-discovery-cycle-severity` clause 2 rules that package roots and
subpackages stay flat sibling Modules, which governs emitted shape rather than
path containment, and suppressing dense ancestors was measured to take AutoDocs
from 2 orphan findings to 20.

## Rationale

The manifest is the boundary the ecosystem already declares, so discovery reads
the package boundary instead of inferring one from file placement. Recursive
counting, the other admissible option, would fix the count while leaving the
anchor to whichever directory the depth bound happened to reach, which is the
behaviour this unit exists to remove.

The supporting rules are not embellishment. Without restarting the depth budget
the manifest is visible but its sources are not, and the AutoDocs case stays
broken. Without innermost-wins a workspace root swallows the packages inside
it, which is the same defect with a coarser blast radius. Without the
no-nesting-inside rule a dropped workspace root returns through its own direct
file count, undoing the drop.

Measured on the pinned AutoDocs commit: orphan findings fall from 12 to 2 and
`webview/packages/shared` maps at its own path. The 2 that remain are a
two-file directory under `MIN_FILES`, which is the threshold working.

## Consequences

- Discovery output is coarser inside a package and better anchored across
  packages: one package, one candidate, whatever its internal shape. A
  repository that declares a manifest per package therefore gets one node per
  package rather than one per dense directory, and refinement splits from
  there.
- Manifest-bearing test fixtures are indistinguishable from product packages.
  Six of the twenty AutoDocs nodes are TypeScript fixture repositories under
  `ingestion/tests/`. Discovery has no signal for that and this ruling adds
  none.
- The manifest list is closed and literal. A repository whose packages are
  declared some other way falls back to the previous direct-count behaviour,
  unchanged.
- A source file sitting directly under a workspace root that was dropped for
  enclosing another package is claimed by nothing and reconciles as an orphan.
  That is the accepted cost of innermost-wins: the alternative is a workspace
  node that owns its packages' files, which is the defect this ruling removes.
- `ratification: local` by the rubric in `dec.decision-ratification-tiers`: the
  node span is the single module `cairn.brownfield`, it supersedes nothing, and
  every path in `affects:` sits outside `docs/registries/binding-surface.md`.
  It is left `proposed` because the loop holds no lens receipts for it.

## Ratification rubric

Deliberately parked at the v0.10.0 cut (2026-08-19), not blocking the release.
The governed behaviour already shipped in #669 with tests, so this decision
records live, verified code rather than gating unshipped work, and nothing waits
on its acceptance. As a `local` ruling it is machine-acceptable only on two
convergent agent-cross-model receipts (`dec.reviewer-panel-ratification`); the
session cutting the release had a single distinct reviewer model available, and
fabricating the second receipt is forbidden. Acceptance is therefore held for a
session with a second distinct model, tracked by
`todo.brownfield-package-root-discovery-ratification`.
