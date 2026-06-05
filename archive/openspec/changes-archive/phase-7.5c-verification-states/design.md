# Design: Phase 7.5c Verification States

## References

- `openspec/conventions.md` section 5 ("Testing Conventions"): authoritative source of the existing `#[ignore = "awaits phase-<N>"]` convention.
- `openspec/specs/testing-baseline/spec.md` requirement "Test-first pre-phase convention": the spec-side declaration of the same convention.
- `AGENTS.md` line 25: the agent-facing instruction that today couples to the `#[ignore]` comment string.
- `openspec/registries/error-codes.md`: the registry where `CC001` is allocated.
- `openspec/changes/phase-8.0-tests/`, `openspec/changes/phase-9.0-tests/`, `openspec/changes/phase-10.0-tests/`: queued pre-phase proposals that will use the new attribute once this phase archives.
- `docs/strongholds/getcairn-cross-check-7.5c.md`: full cross-check that scoped this phase, including the rationale for `CC001` placement and the `cairn-macros/` workspace member decision.

## Workspace Layout

A new Cargo workspace member is added at `cairn-macros/`. The crate is small (around 50 lines of code for the macro implementation, plus a `Cargo.toml` and a `lib.rs`). Its `Cargo.toml` declares `proc-macro = true` and `[lints] workspace = true`. The cairn crate adds `cairn-macros` to its `[dependencies]`. No other workspace member depends on it directly in this phase.

The macro crate is named `cairn-macros` rather than `cflx-macros` because proc macros run at compile time as part of the cairn-the-crate build, not as part of cflx-the-workflow-runner. The user-facing attribute name (`cflx_planned`) is independent of the crate name; the attribute is named for cflx because cflx is the consumer of the planned-verification distinction.

The cairn crate re-exports the attribute via `pub use cairn_macros::cflx_planned;` from `src/lib.rs` so that downstream call sites write `#[cairn::cflx_planned(phase = 8)]` or `use cairn::cflx_planned;` rather than depending on the macro crate by path.

## The `cflx_planned` Attribute

The attribute parses a single named argument: `phase = <integer>`. It emits two effects.

1. The original test function is preserved with an additional `#[ignore = "cflx_planned: phase-<N>"]` attribute prepended. Stacking with an existing `#[ignore]` on the same function is rejected at macro-expansion time with a compile error directing the author to choose one mechanism. This avoids the layered-ignore edge case and keeps the planned-vs-flaky distinction unambiguous.
2. A registration entry is written to a build-derived sidecar at `target/cflx/planned.json`. The entry records the test's fully qualified path (module path plus function name), the target phase number, and the source span (file plus line) of the attribute. The sidecar is regenerated on every build that touches the attribute; it is not committed.

The attribute SHALL accept only the `phase` argument in this phase. Adding new arguments later is forward-compatible because the parser accepts the existing single-argument form unchanged. The attribute SHALL NOT accept `phase = 0` or negative integers; those are rejected at macro-expansion time.

The macro implementation lives in `cairn-macros/src/lib.rs`. It uses `syn` for argument parsing and `quote` for output emission. Both are common proc-macro dependencies and appear elsewhere in the Rust ecosystem; they are added to the workspace dependency list in this phase.

## The `VerificationState` Enum

The enum is defined in `src/verification.rs` (a new file) and re-exported from `src/lib.rs`:

```rust
pub enum VerificationState {
    Draft,
    Planned,
    Passed,
    Failed,
    Blocked,
}
```

Variants map to the operational signals as follows.

- `Draft`: a verification authored but not yet wired to the battery. The pre-`apply` state. Surfaced by absence of any battery integration; not produced by the runtime.
- `Planned`: a verification scoped to a future phase, deliberately skipped now. Surfaced by `#[cflx_planned(phase = N)]` on the test function. The proc-macro is the only producer.
- `Passed`: the test ran and asserted true. Surfaced by `cargo test` returning zero on a non-ignored test.
- `Failed`: the test ran and asserted false or panicked. Surfaced by `cargo test` returning non-zero on a non-ignored test.
- `Blocked`: the test could not execute because of an upstream missing piece (missing fixture, missing upstream phase, missing environment dependency). Surfaced by the cairn runtime returning a `CairnError` carrying the code `CC001`.

The enum derives `Debug`, `Clone`, `PartialEq`, `Eq`, `serde::Serialize`, and `serde::Deserialize`, per the shared type conventions in `openspec/conventions.md` section 4.

## Error Code `CC001`

The code is allocated in `openspec/registries/error-codes.md` under the existing `CC -- Changes` heading. The entry reads:

```
CC001 -- verification blocked by upstream dependency -- phase-7.5c
```

Placement rationale: the consumer of the `Blocked`-versus-`Failed` distinction is `cflx accept` gate logic, and gate logic lives in the change-archival flow, which is the `C` (Changes) category. The `CC` category was empty before this phase. A new category letter (`V` for Verification, `L` for Lifecycle) is overkill for a single code; if a future phase promotes verification to a kernel artefact, that future phase can add a new category and migrate.

The cairn crate adds a new `CairnError` variant (or extends the existing one) so that constructing a blocked-verification error carries the code `CC001`. The variant accepts an upstream-cause field describing what was missing (missing phase, missing fixture, missing environment variable). The cause is rendered in human-readable error output and serialised in JSON error output as a sibling of the `code` field.

## `cflx accept` Gate Integration

`cflx accept` runs the verification battery and inspects results. The integration in this phase is narrow: when the gate encounters a test outcome carrying error code `CC001`, the gate reports the test as `Blocked` rather than `Failed` and exits with a distinct status. A blocked test does NOT cause `cflx accept` to fail the phase by default in this phase; the phase author is responsible for deciding whether the upstream dependency is acceptable for archive. A future phase MAY tighten the gate to fail on `Blocked` by default once operational evidence shows the cases where blocking should be terminal.

The gate consults the sidecar at `target/cflx/planned.json` (when present) to label any `Planned` test outcomes. Tests in the planned set are reported as `Planned` even though `cargo test` reports them as ignored. The gate does NOT execute planned tests under `--ignored` automatically; doing so is the test author's choice via the standard cargo flag.

## Sidecar Format

`target/cflx/planned.json` follows the cairn state-versioning convention from `openspec/conventions.md` section 3. The first field is `version` (integer, starting at `1`). The remaining payload is a list of entries, each with `test_path` (string), `phase` (integer), `file` (string), and `line` (integer). Schema migration follows the standard `migrate_v1_to_v2` pattern in the file's owning module if a future phase changes the layout.

The sidecar is build-derived and not committed. It is rewritten on every build whose macro expansions emit registration entries. Builds that do not touch any `#[cflx_planned]` site leave the sidecar untouched.

## Convention and AGENTS.md Updates

`openspec/conventions.md` section 5 is updated in two places.

1. The "Test-First Pre-Phase" subsection replaces `#[ignore = "awaits phase-<N>"]` with `#[cflx_planned(phase = <N>)]` in the prose and the example. A short note states that the proc-macro expands to `#[ignore]` underneath so `cargo test` keeps working.
2. A new paragraph introduces the five-state enum and points readers to `openspec/specs/testing-baseline/spec.md` for the canonical scenarios and to `openspec/registries/error-codes.md` for `CC001`.

`openspec/specs/testing-baseline/spec.md` requirement "Test-first pre-phase convention" is rewritten in lockstep so the prose, scenarios, and example all match. A new requirement "Verification states attached to test attributes" is added in the same area, with scenarios covering planned, blocked, and the `CC001` surfacing.

`AGENTS.md` line 25 is updated to read: "remove the matching `#[cflx_planned(phase = <N>)]` attribute as the feature lands rather than rewriting those tests from scratch." A second sentence is added: "The attribute is structured (proc-macro), not a comment; do not parse the `#[ignore]` reason string."

## Testing

Tests cover three surfaces.

1. The proc-macro itself: a unit test under `cairn-macros/tests/` that compiles a fixture file containing `#[cflx_planned(phase = 8)]` and asserts that `cargo test` reports the function as ignored, that `cargo test -- --ignored` reports it as failed (panic from an unimplemented body), and that the sidecar at `target/cflx/planned.json` contains an entry with `phase = 8` and the correct test path.
2. The `VerificationState` enum: unit tests that round-trip serialise each variant through `serde_json`, and a parity test that asserts the `Blocked` variant carries error code `CC001`.
3. The gate integration: an integration test that runs `cflx accept` over a fixture phase containing one passed, one ignored-via-`cflx_planned`, and one blocked test, and asserts that the gate output classifies all three correctly.

## Forward Compatibility

The attribute's argument list accepts a single named argument (`phase`) but the parser is structured to accept additional named arguments without a syntax break. A future phase MAY add `infra = "..."` (block when an infrastructure dependency is missing) or `artefact = "<id>"` (block when a referenced artefact is missing) without re-authoring the attribute. This phase neither implements those variants nor reserves them; it only avoids closing the door.

The proc-macro emits `#[ignore]` underneath rather than relying on a hypothetical native skip mechanism. A future phase MAY swap to a native skip if cargo gains one and the migration is worthwhile; the attribute stays the same from the test author's perspective.

## What This Phase Does Not Do

- Does not add a `verification` artefact type to `openspec/specs/artefacts/spec.md`.
- Does not add a new error-code category letter.
- Does not modify the `Makefile` `status-phases` target. That target parses `tasks.md` checkboxes; the attribute migration is orthogonal.
- Does not migrate any existing `#[ignore]` call site (there are none).
- Does not change `cargo test` behaviour from a runner perspective; tests marked `#[cflx_planned]` continue to be skipped because the macro emits `#[ignore]` underneath.
