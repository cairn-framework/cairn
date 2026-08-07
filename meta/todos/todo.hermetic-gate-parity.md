---
node: cairn.root
status: open
created: 2026-08-07
related: [todo.local-gate-attestation, res.chatgpt-issue-audit]
---

# Hermetic gate parity

Replacement unit for the refuted attestation premise
(todo.local-gate-attestation: a perfect receipt saves ~45s against ~159s
CI, per its own measurements). The real gap is parity, not attestation:
local gates and CI can drift in toolchain and command set, which is how
divergence like the convergence-receipt incident stays invisible locally.

## Task

1. Pin toolchains: record the rustc/clippy versions CI uses and make the
   local gate check or print them.
2. Reconcile command sets: `scripts/pre-archive-rust-gates.sh`,
   `cairn hook all`, and the CI workflow should run the same commands or
   document every intentional difference.
3. Remeasure gate wall-times after parity; record the numbers.
4. No attestation work unless a recorded threshold fires after parity.

## Acceptance

- A written parity table (local command vs CI command) with zero
  unexplained differences, plus refreshed timings.
