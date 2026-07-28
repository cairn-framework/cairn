---
node: cairn.root
status: open
created: 2026-07-28
---

# Local Gate Attestation

## Problem, as measured

The premise was that waiting on GitHub CI is a bottleneck worth replacing with a
hermetic local gate that emits a signed receipt. Measurement does not support the
premise at its current size, and it surfaces a different defect worth fixing
regardless.

Measured on 2026-07-28 across nine merged PRs (#511, #513, #515 to #521), each from
the earliest workflow `createdAt` to the latest `updatedAt`:

- Median all-checks-green wall clock 159s, worst 170s (#513). Dogfood was the
  longest job in all nine batches.
- PR #521 specifically (CI run 30363718809): `check` 119s of which clippy 15s,
  tests 81s, rustdoc 3s; `windows-check` 98s; `webui` 32s; `hooks` 8s; Dogfood 155s;
  batch 159s.
- The same deterministic gates locally, warm, on an M2 Max:
  `scripts/pre-archive-rust-gates.sh` 114s, `cargo test` 30s, incremental clippy
  under 2s, fmt under 1s. Roughly 100 to 120s for the warm set.

A perfect local receipt therefore saves about 45s per PR against all-checks-green,
and the warm local pre-archive run takes about as long as the CI `check` job alone.
Any perceived multi-minute wait today came from polling and review round trips, not
from CI duration.

The real defect is that **CI is not hermetic**, so it cannot validate a receipt even
in principle: there is no `rust-toolchain` file and Rust floats through
`dtolnay/rust-toolchain@stable` (`.github/workflows/ci.yml:13-15,32-34,46-48`,
`.github/workflows/dogfood.yml:15-16`), prek installs from `releases/latest`
(`ci.yml:50-55`), runners are `ubuntu-latest` and `windows-latest`, and the webui
visual harness uses an unversioned system Chrome (`ci.yml:74-79`). Biome is pinned
to 2.4.4 and cargo-dist to 0.32.0, which is the exception rather than the rule.

The two gate sets are also not equivalent today: CI runs `cargo test --locked
--workspace` while the local script omits `--locked`, and the local script passes
`RUSTFLAGS="-D warnings"` where CI passes `-- -D warnings` to clippy
(`.github/workflows/ci.yml:21-24`, `scripts/pre-archive-rust-gates.sh:7-17`). That
is worth knowing independently of attestation.

## Scope

Phase 1, do now, cheap and useful either way:

- Pin the toolchain: add `rust-toolchain.toml` with an exact version and the
  components CI needs, and use it in every workflow so local and CI compile with the
  same rustc.
- Pin prek, the runner images where practical, and the Chrome and Node versions the
  webui harness depends on.
- Reconcile the two gate invocations so `scripts/pre-archive-rust-gates.sh` runs the
  same commands as CI `check`, `--locked` included, or documents each deliberate
  difference inline.
- Re-measure the CI batch and the warm local set after pinning, and record both
  numbers in this todo.

Phase 2, only if Phase 1's re-measurement shows a gap worth paying for, for example
if the batch grows past several minutes or the matrix widens:

- A content-addressed hermetic runner (Nix flake or equivalent) taking the tree as
  input and emitting a signed attestation of `{commit_sha, tree_hash,
  gate_set_hash, toolchain_hash, result}`. Both the commit SHA and a canonical tree
  hash are bound: the tree hash proves what was gated, the commit SHA proves which
  head claimed it, and a receipt matching one but not the other is refused.
- A trust model, stated before any code: which keys are authorised, where the public
  set lives (a checked-in allowlist, so a key change is a reviewed diff), how
  rotation and revocation work, and what happens to receipts signed by a key that
  was later revoked. A signature alone proves that some key signed matching content,
  not that an authorised reproducible gate produced it.
- A trivial GitHub check that verifies the signature against the allowlist, verifies
  all four fields against the PR head, and skips the deterministic jobs only on a
  full match. Anything else falls through to the full matrix rather than failing, so
  a receipt can never be the reason a gate is missed.

## What a receipt could and could not cover

Skippable on a sound receipt: CI `check` (fmt, clippy, tests, rustdoc), the
file-size gate, the deterministic parts of `hooks` once prek is pinned, and the
static webui audits (Biome, design tokens, a11y) once Node and Biome are pinned.
Dogfood is tree-derived in intent but rebuilds and installs the binary, so it is
skippable only if the receipt reproduces that environment too.

Never skippable: `windows-check`, any release platform or cross-build matrix member,
and the webui visual harness, which needs a real browser. Platform coverage cannot
be attested from an M2.

## Depends on

Nothing for Phase 1. Phase 2 depends on Phase 1's re-measurement and on a decision,
since skipping a required check changes what a green PR means.

## Acceptance

Phase 1:

- `rustc --version` is identical locally and in every CI job, driven by a checked-in
  toolchain file.
- `scripts/pre-archive-rust-gates.sh` and CI `check` run the same commands, or every
  difference is documented at the line that causes it.
- Re-measured CI batch median and warm local duration are recorded in this todo,
  replacing the numbers above.

Phase 2, if entered:

- A receipt is refused when the tree hash, gate set hash, or toolchain hash differs
  from the PR head, proven by a test that tampers with each of the three.
- `windows-check` and the platform matrix still run on every PR regardless of any
  receipt.

The audit trail behind the timings, including its window, sources, and limits, is
recorded as a comment on PR #523. Workflow run ids are quoted inline above so the CI
numbers can be re-pulled from GitHub without it.

## Origin

Maintainer conversation, 2026-07-28: run the same CI locally where the machine is
faster, in a way that meets a standard and can offer proof, Nix based, so the GitHub
run can be skipped. The measurement above narrows that to pinning first and keeps
the attestation as a conditional second phase. Cost and pinning audit run the same
day; its run IDs are quoted inline above so the numbers stand without the transcript.
