---
node: cairn.tests
status: done
created: 2026-07-17
---

# Oversized Test File Baseline

## Problem

`CAIRN_MODULE_OVERSIZED` was Info because the scan pipeline checks all
node-owned claimed files, while the shell size gate discovers Rust only
under `src/` and walks JS/CSS under blueprint-declared paths; it never
discovers `tests/*.rs`. The `cairn.tests` node originally had seven
oversized, unmarked Rust test files outside that shell discovery scope:

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

## Resolution (2026-07-17)

Six files carry a `cairn:allow-large-module` marker with a file-specific
reason: `tests/finding_code_coverage.rs`, `tests/kernel.rs`,
`tests/phase_10_distribution.rs`, `tests/phase_7_6_ai_provenance.rs`,
`tests/phase_7_7_ux_foundation.rs`, `tests/phase_9_brownfield.rs`.
`tests/output_token_efficiency.rs` was split at its existing Part A/Part B
seam into `tests/output_token_efficiency_findings.rs` (434 lines) and
`tests/output_token_efficiency_status_brief.rs` (269 lines), both under the
500-line limit, so no marker was needed there; every relocated test body
and name is preserved verbatim.

With the baseline clear, `CAIRN_MODULE_OVERSIZED` severity was promoted
from Info to Warning in `src/map/module_size.rs`. `cairn scan --strict`
stays green on this clean checkout with zero `CAIRN_MODULE_OVERSIZED`
findings; Warning now blocks any newly introduced unmarked oversized
claimed file project-wide.
