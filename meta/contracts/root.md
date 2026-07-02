---
node: cairn.root
---

# Contract: cairn.root

## Purpose

Crate entry points and cross-cutting primitives. Owns the binary `main`, the
library root that re-exports every subsystem module, the shared `CairnError`
type, the `VerificationState` lifecycle enum, and POSIX signal handling. This is
the seam between the operating system (process start, signals, exit codes) and
the typed services in the rest of the crate.

## Public interface

- `lib.rs`: the crate root. Declares and re-exports the public module tree
  (`artefacts`, `blueprint`, `map`, `scanner`, `query_api`, `cli`, and the rest)
  and exposes `package_name()`, `package_version()`, and `version_label()`.
- `error.rs`: `CairnError`, the unified public error type. Every public API
  returns `Result<T, CairnError>`; `.code()` yields the registry error code.
- `verification.rs`: `VerificationState` (`Draft`, `Planned`, `Passed`,
  `Failed`, `Blocked`), serde-serialisable, used to classify test state.
- `signal.rs`: SIGINT handling without a Foundation/ObjC dependency.
- `report.rs`: `ISSUE_BASE`, `issue_url()` (prefilled GitHub issue links,
  always opened by the user, never sent automatically), and
  `install_panic_hook()` (crash report + issue link on stderr, then defers
  to the previously installed hook so `RUST_BACKTRACE` output is preserved).
- `main.rs`: the `cairn` binary entry point; dispatches into `cli`.

## Invariants

- The public boundary unifies on `CairnError`; internal modules may use narrower
  error types but the crate surface does not leak them.
- `version_label()` is deterministic: `"{package_name} {package_version}"`.
- `VerificationState` round-trips through serde without loss.
- Signal handling stays dependency-free (no Foundation/ObjC), per the module
  doc rationale.

## Dependencies

Leaf with no outgoing blueprint edges. `main.rs` dispatches into `cairn.kernel.cli`
at the binary layer; the library root re-exports all sibling modules but holds no
domain logic of its own.

## Tests

Unit tests colocated in `src/lib.rs` (version label and re-export surface),
`src/verification.rs` (serde round-trip across all five states), and
`src/report.rs` (`issue_url` percent-encoding and label presence). The error
type is exercised transitively by every subsystem that returns `CairnError`.
The panic hook is verified manually (crash heading, prefilled link, and
transparency line on stderr) since spawning a real panicking process is not
worth an integration test.
