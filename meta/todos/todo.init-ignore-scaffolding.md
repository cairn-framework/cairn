---
node: cairn.brownfield
status: open
created: 2026-07-16
---

# Init-Time Ignore Scaffolding

Implement the init-time ignore assistance already specified in
docs/spec.md section 6.1: `cairn init` (especially `--from-code` on
brownfield repos) scans the project, proposes an initial ignore list, and
the human confirms before it is written, removing the "hundreds of orphan
warnings on first run" failure mode.

Implementation note from `res.codeatlas-analysis` (finding 9): reuse the
15 `IGNORE_PATTERNS` heuristics in `src/brownfield/onboard.rs` as the
shared source of truth rather than introducing a second hardcoded list.
Scan-time merging of .gitignore and .cairnignore is already correct and
root-resolved (`src/scanner/config/mod.rs`); this todo is only about the
init-time proposal step.

## Mission disposition

2026-08-02: keep against dec.cairn-mission. Serves fit-for-purpose. It is campaign filler A on a disjoint surface and keeps adoption work moving.
