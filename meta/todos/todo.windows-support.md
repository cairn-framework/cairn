---
node: cairn.root
status: done
created: 2026-07-03
---

# Windows Support

Deferred at v0.1.0 launch: signal-hook (Cargo.toml, used in src/signal.rs
for SIGINT) is POSIX-only, so v0.1.0 prebuilt releases skip
x86_64-pc-windows-msvc.

Resolved: moved `signal-hook` to a Unix-only `[target.'cfg(unix)'.dependencies]`
table in Cargo.toml and cfg-gated `src/signal.rs`
(`#[cfg(unix)]`/`#[cfg(not(unix))]`) with a no-op stub on non-Unix (OS
default Ctrl-C still applies; the stub is infallible, so the error path
both callers do have on real registration failure (`src/ui/mod.rs`
propagates via `?`, `src/cli/commands/watch.rs` returns a hard CLI error)
is simply unreachable on non-Unix, not tolerated by the callers). Added
`x86_64-pc-windows-msvc` back to `dist-workspace.toml` targets plus a
`powershell` installer, verified via `dist plan`. Added a `windows-check`
CI job (native `windows-latest`, `cargo check` + `cargo clippy -D warnings`
both `--all-targets --all-features`, for parity with the `check` job's
clippy coverage on the `#[cfg(not(unix))]` stub) and cross-checked
`x86_64-pc-windows-gnu` locally.

bd:cairn-omf
