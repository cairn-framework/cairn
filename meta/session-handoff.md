# Session Handoff: 2026-06-23 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done

- **`cairn-t59` shipped** (PR #141): resolved the graph-root-fingerprint spike
  ("Dolt is git for SQL; should cairn be git for a knowledge graph?").
  - New decision `meta/decisions/graph-root-fingerprint.md`
    (`dec.graph-root-fingerprint`, nodes `cairn.kernel.scanner` +
    `cairn.reconcile`). Ruling:
    1. **Reject a versioned graph store** (Dolt-analogue): the reconciled graph
       is derived, not authored; versioning a projection is the two-source-of-
       truth trap.
    2. **Close the real gap: gate dependency-edge drift.** Verified that declared
       dependency edges are endpoint-validated only (`map/build.rs`
       `validate_edges`) and rebuilt in-memory each scan; `BlueprintSnapshot`
       records nodes only (`scanner/state.rs:65-70`) and the change gate iterates
       node add/remove/parent/kind only (`scanner/checks.rs:48-67`). So changing
       a node's declared edge set is a structural change with no covering-decision
       gate. Fix via the existing snapshot/finding pattern (per-node granularity).
    3. **Defer the aggregate graph-root fingerprint and any commit-binding:** 0
       consumers in `src/`, and `DefaultHasher` (`reconcile/fingerprint.rs:23`,
       SipHash) is toolchain-unstable, disqualifying a committed cross-machine
       value. Recompute-and-compare at per-node/per-target granularity stays the
       gate.
  - **Pre-submit `/debate` reshaped the decision.** Two opposing `oracle`
    steelmen ran; the conservative landed two verified structural catches (the
    aggregate root is a consumer-less premature abstraction; a DefaultHasher
    committed trailer is toolchain-unsafe), both adopted, and surfaced the real
    win (dependency-edge drift is uncovered). The reject-store ruling survived.
  - **`cairn-9v1` filed** (P3, maintainer-gated, discovered-from cairn-t59):
    implement the dependency-edge-drift gate. The aggregate root + trailer remain
    a recorded, deferred option, NOT filed for build.

## Current State

- `cairn lint` / `cairn scan` clean (0 findings) on `main`.
- 5 ready beads, all **P3**, all now **maintainer-directed**:

  | Bead | Unit | Why it needs George |
  | --- | --- | --- |
  | `cairn-9v1` | implement dependency-edge-drift gate | implementation, gated on greenlight |
  | `cairn-1w3` | spike: warn when a toolchain's lint isn't strict | product-scope call (project-health is one layer beyond architecture) |
  | `cairn-2z9` | spike: beads as first-class task layer | bends the markdown-artefact invariant (spec.md:11) |
  | `cairn-y1m` | spike: bead<->GitHub status sync | accept a second source of truth (maintainer call) |
  | `cairn-y7p` | browser UI/UX iterative AI fix loop | large multi-iteration implementation + tooling choice |

- No open PRs. No P0/P1/P2.

## Next: maintainer-directed

`cairn-t59` was the one remaining spike whose ruling was a pure technical
decision (verdict already reached, reinforcing the accepted no-second-source-of-
truth principle), so the loop resolved it autonomously and kept implementation
deferred. Every remaining ready bead is either a maintainer-gated implementation
(`cairn-9v1`, `cairn-y7p`) or a spike whose ruling is a product/scope call
reserved for George (`cairn-1w3`, `cairn-2z9`, `cairn-y1m`). The loop should
resume once George greenlights a specific unit (converts a spike to a coding unit,
or greenlights `cairn-9v1`) or files new work.

## Agent Entry Points

- `cairn context`; `cairn get <id>` / `cairn neighbourhood <id>` for detail.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
