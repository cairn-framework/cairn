---
node: cairn.brownfield
status: done
created: 2026-07-11
---

# Fix Flaky test_complete_session_writes_genesis

`brownfield::interview::tests::test_complete_session_writes_genesis` fails
intermittently in the full workspace suite (observed twice on 2026-07-11
during `cairn change accept` runs) and passes in isolation and on rerun.
Failure: `assertion failed: !dir.join("research/interview-session.json").exists()`
at src/brownfield/interview.rs - the session file exists when the test
expects it cleaned up.

Smells like test-isolation leakage: a shared/reused temp directory or
ordering-dependent interaction with another test writing the same path.

Task: reproduce under full-suite ordering (e.g. `cargo test --workspace`),
find the shared state, isolate it (own temp_root per test, or cleanup
guard), and prove stability across repeated full-suite runs.

Why it matters: the accept gate runs the full suite; a probabilistic
failure trains the loop to rationalise "pre-existing flaky, wave it
through", eroding the gate.

Acceptance: full workspace suite green across several consecutive runs;
the test no longer depends on execution order.

## Resolution (2026-07-16)

Root cause: the interview test helper used a fixed `$TMPDIR/{name}` path, so overlapping test-process invocations shared `bf-int-genesis/.../research/interview-session.json` and raced during write/delete. Eight concurrent pre-fix invocations reproduced the exact assertion failure in 2/8 runs.

Fix: `temp_change_dir` now returns an owning `tempfile::TempDir` plus the change path. All seven call sites retain the guard for the test duration, giving each test a fresh randomly named root with automatic cleanup. This is an isolation-only change; production interview behaviour is unchanged.

Verification: the fixed test passed 8/8 concurrent invocations with 0 failures across 20 stress invocations. The full interview module passed 7/7 tests. Five full workspace runs each had all 1267 library tests green, including every interview test; the only failures were the three pre-existing `command_reference_consistency` tests caused by the CLAUDE.md consolidation on main, which the parent is handling separately. `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo run --release --bin cairn -- scan --strict` exited 0; scan left `map.json` unchanged.
