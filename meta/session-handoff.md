# Session Handoff — 2026-05-01

## What Was Done This Session

### 1. Spec Purpose fixes (commit `c1e8d34`)
- Swarm-generated `## Purpose` sections for all 12 canonical specs.
- 12 constructive + 12 adversarial subagents; 5 semantic issues caught and fixed.
- `openspec validate --specs`: 15/15 pass.

### 2. `.0-tests` pre-phase proposals (commits `518a9c7`, `1109c15`, `0f1927d`)
- Created `phase-7.6.0-tests` (24 test stubs for AI Provenance Foundation).
- Created `phase-7.7.0-tests` (28 test stubs for UX Foundation).
- Created `phase-7.8.0-tests` (8 test stubs for Cairn Export).
- Adversarial review swarm caught and fixed:
  - `#[ignore]` → `#[cflx_planned(phase = 76/77/78)]`
  - `todo!()` → `unimplemented!()` (workspace denies `clippy::todo`)
  - Doc-comment requirements (`//!` crate docs, `///` per test)
  - 7.6.0 scenario count fix (27 scenarios, 24 unique tests)
  - Sed damage in 7.7/7.8 test names fully corrected.

### 3. Archive gate (`src/cli/accept.rs`)
- Reverted direct `openspec validate --specs` invocation from `accept.rs`.
- Added `openspec validate --specs --strict` to `.pre-commit-config.yaml` pre-push hook instead.

### 4. Conventions (`openspec/conventions.md` §5)
- Reverted `MUST` to `SHOULD` (avoid retroactive invalidation of archived phases).
- Removed arbitrary >10 tasks threshold.
- Removed manual-ignore fallback loophole.
- Fixed decimal phase encoding to zero-padded injectivity (`phase = 706` for 7.6).
- Documented macro restriction on combined `#[ignore]`.

### 5. Deep-review skill
- Created `~/.kimi/skills/deep-review/` for future autonomous constructive + adversarial review swarms.

---

## Review Findings (Claude + Codex headless)

Full findings live at:
- `meta/reviews/claude-findings.md` — design, architecture, policy review
- `meta/reviews/codex-findings.md` — code, compilation, test-structure review

### Critical Issues (must fix before next phase apply)

| # | Issue | Owner | Path |
|---|---|---|---|
| 1 | **Phase encoding mismatch**: pre-phases use `phase = 76/77/78`; conventions.md mandates `phase = 706/707/708` | This session's debt | `openspec/changes/phase-7.{6,7,8}.0-tests/` |
| 2 | **`cairn-macros/tests/planned_attribute.rs` fails compilation** under workspace lints (`missing_docs`, `clippy::assertions_on_constants`, `dead_code`) | Pre-existing (phase-7.5c) | `cairn-macros/tests/planned_attribute.rs` |
| 3 | **Actual `.rs` test files never created** — only OpenSpec proposals exist | By design, but needs apply | `tests/phase_7_6_ai_provenance.rs` (et al.) |

### Major Issues

| # | Issue | Path |
|---|---|---|
| 4 | Test naming: 7.7 uses `check__`/`explorer__` prefixes without `test_`; 7.8 uses bare names | `phase-7.7.0-tests/`, `phase-7.8.0-tests/` |
| 5 | `CC002` referenced in 7.6.0-tests but not registered in `openspec/registries/error-codes.md` | `openspec/registries/error-codes.md` |
| 6 | Legacy pre-phases (8.0, 9.0, 10.0) still use manual `#[ignore]` / `todo!()` — policy drift | `openspec/changes/phase-{8,9,10}.0-tests/` |
| 7 | `src/cli/accept.rs` dead code: `read_planned_tests()` result is discarded | `src/cli/accept.rs` |
| 8 | `cargo test --workspace` and `cargo clippy --workspace` are not in pre-push hooks; macro test failures hidden | `.pre-commit-config.yaml` |

### Minor Issues

- `accept.rs` uses `std::path::PathBuf` instead of `camino::Utf8PathBuf` (cosmetic).
- `cairn-macros/src/lib.rs` hardcodes `file: "<unknown>"`, `line: 0` despite `span-locations` feature.
- `openspec-validate-specs` hook uses bare `openspec` prefix while `accept.rs` uses `cflx` prefix (cosmetic).

---

## Current State

- Branch: `dev`
- Working copy: clean
- `openspec validate --changes --strict`: 13/13 pass
- `openspec validate --specs --strict`: 15/15 pass
- `cargo check`: pass (root only)
- `cargo clippy --all-targets --all-features -- -D warnings`: pass (root only)
- `cargo test --no-run --workspace`: **FAIL** (macro test compilation)

---

## Next Steps for Fresh Session

1. **Fix `cairn-macros/tests/planned_attribute.rs` compilation** — add `//!` doc, remove tautological `assert!(true)`, handle dead code.
2. **Fix phase encoding** — update all `phase = 76/77/78` references to `706/707/708` across the three pre-phase proposals.
3. **Register `CC002`** in `openspec/registries/error-codes.md` (or defer reference to phase-7.6 apply).
4. **Normalize test names** — add `test_` prefix to 7.7 and 7.8 test names; align 7.6 design.md mapping table.
5. **Add `--workspace` to pre-push hooks** so macro tests are built and linted.
6. **Optionally apply the three pre-phases** — write the actual `tests/*.rs` stub files and commit them.
7. **Optionally update legacy pre-phases (8.0, 9.0, 10.0)** to use `#[cflx_planned]` instead of manual `#[ignore]`.

---

## Files Touched This Session

```
openspec/changes/phase-7.6.0-tests/          (created)
openspec/changes/phase-7.7.0-tests/          (created)
openspec/changes/phase-7.8.0-tests/          (created)
openspec/conventions.md                      (modified §5)
src/cli/accept.rs                            (reverted openspec addition)
.pre-commit-config.yaml                      (added openspec-validate-specs hook)
~/.kimi/skills/deep-review/                  (created)
meta/reviews/claude-findings.md              (created)
meta/reviews/codex-findings.md               (created)
meta/session-handoff.md                      (this file)
```

## Key Decision Logs

- `docs/strongholds/getcairn-cross-check-7.5c.md` — `#[cflx_planned]` MUST replace manual `#[ignore]`
- `docs/strongholds/oq1-reconciliation-validate.md` — `cflx openspec validate` never iterates `openspec/specs/`
- `docs/strongholds/tdd-posture-investigation.md` — do NOT add TDD; keep test-first pre-phase + gate-enforced coverage
