# Session Handoff: 2026-06-29 (cairn-loop)

Branch: `main`, working tree clean. Local `main` == `origin/main`.

## What Was Done (this session)

Two units shipped through the full loop (branch -> gates -> two-lens presubmit
review -> PR -> CI -> squash-merge -> close -> log):

1. **PR #191 merged (`cairn-bss`)**: reconciled `AGENTS.md` task-tracking
   guidance. The `bd setup`-generated blocks asserted a universal "use bd for
   ALL task tracking / do NOT use TodoWrite" mandate that contradicted the
   ratified model. Resolution (grounded, not memory):
   - `docs/spec.md` §8.2 "Todo (authority)" makes native Todo a kernel artefact;
     §5 says project workflow conventions belong in AGENTS.md, not the framework.
   - `dec.no-orchestrator`: Beads = optional Layer-1 storage. `dec.bd-upgrade-plan`:
     "this repo tracks work in beads". `dec.native-task-state-and-agent-guidance`
     ruling 2 directed this reconciliation.
   - Added a hand-authored "## Task tracking" section outside the `bd setup`
     markers and reconciled the two in-marker Rules lines in place (the "pin"
     half of ruling 2's pin-or-regenerate). Node-linked beads surface via
     `cairn backlog <node>` (not `--include-todos`, which renders native Todo).

2. **PR #192 merged (`cairn-9ey`, P2 bug)**: the pre-push dogfood gate could
   false-green on a stale PATH binary. `scripts/dogfood.sh` invoked bare `cairn`,
   resolving via PATH to `~/.cargo/bin/cairn` instead of the working tree's
   build. Fixed to run cairn via `cargo run --release --bin cairn` (builds and
   runs exactly that binary; respects `CARGO_TARGET_DIR`). Regression guard:
   `tests/dogfood_gate.rs`. Verified: the pre-push hook now surfaces the repo
   binary's findings where the stale global hid them.

## Decision-model note (no code change)

Native Todo (spec §8.2) is the artefact authority; bd is this repo's optional
Layer-1 tracker (opted in for its richer workflow state). The per-node beads
view (`src/state/backlog.rs`, `cairn backlog <node>`) already surfaces
`cairn-node:`-labelled beads without minting Todo files, so 0 native todos is
the design working as decided, not a gap.

## Open backlog (3 ready, all design/decision-gated)

Filed this session from a gap audit. None is a clean surgical fix; each needs a
maintainer steer before code:

- **cairn-1me** (P3): wire `meta/changes` into the scan `ArtefactSet`/reconcile
  graph. Substantial, and **overlaps the deferred `cairn-9w9`** (this is the
  correlator substrate). Do NOT auto-build: it pre-builds deferred work.
  Change-loading infra already exists (`src/changes/discover()` / `load_change()`).
- **cairn-tzi** (P3): `Decision.revisited` is parsed/rendered but never written
  (0/38 populated). It is a **spec field** (spec §8.3), so removal deviates from
  spec; wiring needs a design call (set-on-archive when a change touches a
  decision, or a `cairn decision revisit <id>` command). Recommend: wire it.
- **cairn-b96** (P4): spec-rules registry has no "feasible-but-deliberately-
  deferred" status (only enforced/pending/declared); also verify the 634
  (`pending`) vs 635/636 (`declared`) tiering is principled. Registry-model
  decision + a quick investigation.

## Deferred (unchanged)

- **cairn-9w9** (P3, deferred until 2026-07-11): revisit-trigger relevance
  correlator (spec:634). Maintainer chose honest deferral; the signal is
  lagging/redundant at current scale. Resume only on maintainer go.

## Agent Entry Points

- `cairn context` / `cairn next` / `cairn backlog <node>` / `cairn rationale <id>`.
- Dev loop: `docs/agent/cairn-dev-workflow.md` via `/cairn-loop`. `bd ready` for work.
