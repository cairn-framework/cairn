---
node: cairn.kernel.cli
status: open
created: 2026-07-12
---

# Accept Language Aware Gates

gh:#234

`cairn change accept` still hardcodes cargo gates, so non-Rust changes can never
pass acceptance.

## Evidence (verified on main, 2026-07-12)
- `src/cli/accept.rs:10-55` unconditionally runs `cargo build`, `cargo clippy`,
  `cargo fmt`, and `cargo test --workspace --locked`. No `gates:` config section,
  no language detection, no CLI override.
- Probed in a scratch bun/TS project: `cairn change accept --json` returned
  `gate_outcome: failed` with every step failing on missing `Cargo.toml`.

## Task
Make the accept gate language-aware: derive gate commands from project language
(the generic reconciler already infers it) or a `gates:` section in
`cairn.config.yaml`, falling back to cargo for Rust projects. Non-Rust projects
must have a supported path to pass acceptance.
