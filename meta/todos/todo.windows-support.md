---
node: cairn.root
status: open
created: 2026-07-03
---

# Windows Support

Deferred at v0.1.0 launch: signal-hook (Cargo.toml, used in src/signal.rs
for SIGINT) is POSIX-only, so v0.1.0 prebuilt releases skip
x86_64-pc-windows-msvc. Fix is cfg-gating src/signal.rs with a
Windows-native alternative (e.g. ctrlc crate or windows console handler)
and adding the target back to dist-workspace.toml.

bd:cairn-omf
