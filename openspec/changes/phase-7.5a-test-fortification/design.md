# Design: Phase 7.5a Test Fortification

## References

- `docs/spec.md` sections 4 (reconcile) and 11 (hooks) — test discipline is part of the spec's notion of guarded change.
- `openspec/conventions.md` — module size limits and existing testing conventions.
- `openspec/changes/archive/phase-7-mcp/` — where the wire format was stabilised but no snapshot lock was added.

## Snapshot Crate

Use `insta` (not `expect-test`). Reasons: explicit review workflow (`cargo insta review`), mature JSON support via `assert_json_snapshot!`, first-class file-based snapshots alongside tests, and the de-facto Rust standard.

Dev-dependency:

```toml
insta = { version = "1", features = ["json", "yaml"] }
```

Snapshot layout:
- File-based snapshots under `tests/snapshots/` for `/api/*` responses (serde-normalised JSON).
- Inline snapshots for small value checks inside `#[cfg(test)] mod tests` blocks where round-tripping through a file adds no value.

## Per-Module Test Placement

Inline `#[cfg(test)] mod tests` inside each file. This is permitted by `openspec/conventions.md` and keeps tests physically adjacent to the code under test. When the same module is split in Phase 7.5b, the test module splits with it; each new submodule inherits its slice with minimal reorganisation.

## Coverage Targets

| File | Test focus |
|---|---|
| `src/ui.rs` | Route dispatch table, 404 behaviour, content-type, `schema_version` field presence, method filtering. |
| `src/cli/mod.rs` | One test per top-level command verb exercising both human and `--json` output surfaces, plus the primary error path. |
| `src/changes.rs` | Delta parse → reconcile → serialise round-trip for ADDED / MODIFIED / REMOVED / RENAMED. Conflict-detection branch. |
| `src/query_api.rs` | Public function boundary tests; at least one positive and one negative case per exported API. |
| `src/artefacts/registry.rs` | Register / lookup / unknown-kind / duplicate-kind cases. |

Snapshot review is manual during this phase: `cargo insta review` must be run by the implementor before commit. The snapshots in the initial commit represent "current behaviour, inspected and accepted." This is the baseline Phase 7.5b refactors against.

## File-Size Gate

`scripts/check-file-sizes.sh` is a ~20-line POSIX shell script:

- Iterates `src/**/*.rs` excluding `target/`.
- Reads the first non-blank line of each file. If it matches `^// cairn:allow-large-module reason: .+$` the file is allow-listed.
- Counts lines; exits 1 with an enumerated report when any non-allow-listed file exceeds 500 lines.

Wired into `scripts/pre-archive-rust-gates.sh` as the last check (after fmt, clippy, test). A module that passes all functional gates still cannot archive if the size convention regressed.

The allow-list mechanism is load-bearing: the five current god modules carry `// cairn:allow-large-module reason: scheduled-for-phase-7.5b-split` at the top so this phase archives green. Phase 7.5b removes them as each module drops below the ceiling.

## CFLX Sort Script

Current regex: `^phase-(\d+)-`. Extended to `^phase-(\d+)(?:\.(\d+))?([a-z]?)-`. Sort key becomes `(major: int, minor: int, suffix: str)`; non-matching change ids continue to sort after matched ones.

Dependency chaining is unchanged: previous phase in sorted order is the dependency.

Resulting order with this phase in place: `phase-7.5a-test-fortification` → `phase-8-summariser` → `phase-9-brownfield` → `phase-10-distribution`. Phase 7.5b will slot between 7.5a and 8 when drafted.

## Test-First Pre-Phase Convention

Add to `openspec/conventions.md`:

> Feature phases that introduce new acceptance criteria SHOULD be preceded by a paired pre-phase `phase-<N>.0-tests` whose apply task writes failing test assertions against the feature's acceptance criteria. Pre-phase tests MUST be marked `#[ignore = "awaits phase-<N>"]` so pre-phase archives pass `cargo test` cleanly. Phase N's first task group MUST remove the `#[ignore]` attribute per test as the corresponding feature code lands.

The `#[ignore]` convention resolves the chicken-and-egg of "pre-phase must archive while its tests are currently failing." Ignored tests run on demand via `cargo test -- --ignored`; they are committed, diff-visible, and act as the design contract the feature phase is graded against.

`AGENTS.md` receives a one-paragraph pointer: "When implementing a feature phase, check whether a paired `phase-<N>.0-tests` pre-phase has landed. If so, your first task in each group is to remove the `#[ignore]` attribute from the relevant test and make it pass."

## Testing The Meta-Gates

- `scripts/check-file-sizes.sh` gets a POSIX shell-based self-test in `tests/scripts/check-file-sizes.sh` (or a small Rust integration test invoking it via `std::process::Command`) covering: file at exactly 500 passes, at 501 fails, allow-list comment honoured, missing reason rejected.
- `scripts/cflx-analyze-cairn-phases.py` gets a `python -m unittest` or docstring doctest covering decimal ordering, suffix ordering, and stability for plain integer phases.

## Out-of-Band Review

This phase's PR is eligible for an orchestrator-run Watcher + Reforger pass before merge to catch obvious review gaps. That discipline is not encoded in `.cflx.jsonc` yet; the separate review-gate infrastructure is a future convention change, not in scope here.
