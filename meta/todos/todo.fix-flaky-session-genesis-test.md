---
node: cairn.brownfield
status: open
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
