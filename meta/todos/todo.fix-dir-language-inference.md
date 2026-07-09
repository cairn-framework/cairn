---
node: cairn.kernel.scanner
status: open
created: 2026-07-09
satisfies: fix-dir-language-inference
---

# Fix directory language inference defaulting to Rust

A module `path "./lib"` pointing at a non-Rust directory is silently
inferred as Rust (`src/scanner/mod.rs:96` `unwrap_or(Language::Rust)`),
discovers zero files, and reports a stable hash as if reconciled. Make an
undeterminable language a visible state: infer from directory contents,
else emit a warning and skip reconciliation. Suppress the hash when a
target claims zero files. See `meta/changes/fix-dir-language-inference/`.
