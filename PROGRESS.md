# Cairn Atomic Improvement Loop

## Cadence Definition

The **Atomic Improvement Loop** is a repeatable process for continuously improving the cairn codebase:

- One self-contained improvement per iteration.
- Each iteration: re-orient from this file, select the next highest-value atomic item, implement, validate, commit, update state.
- Validation: `scripts/pre-archive-rust-gates.sh` and `cairn lint` must pass before commit.
- Bead tracking: create/close a bead via `bd` and export `.beads/issues.jsonl` on each completed item.
- Stop when the session budget is reached, marginal value drops, or the next item is too large for one atomic commit.

## Selection Heuristic

1. Run `bd ready` to see open work.
2. Pick the highest-value item that fits the atomic criteria:
   - small scope,
   - independently committable,
   - passes gates locally.
3. If the highest-value item is repo-wide, multi-file migration, or requires deep cross-cutting changes, escalate it to a focused session instead of trying to squeeze it into the loop.

## What This Cadence Is Good For

- Unit tests for isolated helpers.
- Module-level documentation.
- Clippy/refactor cleanups.
- Small, low-risk mechanical improvements.

## What This Cadence Is NOT Good For

- Repo-wide architectural changes.
- Multi-file schema migrations.
- Bugs whose root cause spans multiple subsystems.

## Last Completed Session

- **Commits**: 113 atomic improvements (module splitting, test extraction, unit tests, module docs, clippy cleanups).
- **Tests**: 880 lib tests pass; integration / phase tests pass.
- **Gates**: `scripts/pre-archive-rust-gates.sh` passes; `cairn lint` clean.
- **Merge to main**: all work merged into `main` and pushed; GitHub default branch set to `main`.

## Notes from Last Session

- The pre-push dogfood gate (`cairn lint`, `cairn hook all`) caught an issue that the per-iteration gate did not: collapsed interface hashes. Add this to validation when touching scanner, reconcile, or hook code.
- The Atomic Improvement Loop deliberately left large, repo-wide work for focused sessions rather than fragmenting it.
