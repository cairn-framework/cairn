---
node: cairn.tests
status: open
created: 2026-07-17
---

# Oversized Test File Baseline

## Problem

`CAIRN_MODULE_OVERSIZED` is currently Info because the scan pipeline checks
all node-owned claimed files, while the shell size gate only discovers `src/`
and `src/ui_assets/`. The `cairn.tests` node has seven oversized, unmarked
Rust test files outside that shell discovery scope:

- `tests/finding_code_coverage.rs`
- `tests/kernel.rs`
- `tests/output_token_efficiency.rs`
- `tests/phase_10_distribution.rs`
- `tests/phase_7_6_ai_provenance.rs`
- `tests/phase_7_7_ux_foundation.rs`
- `tests/phase_9_brownfield.rs`

## Resolution options

Choose one option per file during the baseline pass:

1. Add a first-line `// cairn:allow-large-module reason: ...` marker with a
   specific, reviewable reason when the file's cohesion is intentional.
2. Split the file into cohesive modules and remove the need for an exemption.

After the seven-file baseline clears, revisit `CAIRN_MODULE_OVERSIZED`
severity toward Warning. Keep Info while these findings are expected on a
clean repository, so `cairn scan --strict` remains non-blocking.
