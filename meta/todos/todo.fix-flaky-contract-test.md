---
node: cairn.kernel.cli
status: done
created: 2026-07-08
---

# Fix flaky contract JSON test

`src/cli/mod.rs::test_contract_and_order_json_served_by_query_api` fails
intermittently under parallel `cargo test` but passes single-threaded
(`--test-threads=1`). The failure is `contract --json app.api` returning
non-zero; the `contract` branch (`src/cli/mod.rs:361`) reads `scan_result`,
not `changes_dir`, so it is unrelated to the `--changes-dir` fix (`daffa6f`).

Observed while resuming the dev loop after #206. CI passed `daffa6f` (lucky
ordering), but local parallel runs hit it. Root cause is a concurrency /
isolation bug in the cli test binary (shared CWD or file contention, or scan
non-determinism). The loop cannot trust `cargo test` until this is
deterministic.

Acceptance: the test passes reliably under default parallel `cargo test`
(including CI) with no single-threaded workaround.
