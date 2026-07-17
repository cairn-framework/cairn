---
node: cairn.root
status: open
created: 2026-07-17
---

# Cargo Include Anchoring


## Problem

Cargo `include` patterns in Cargo.toml are gitignore-style and unanchored, and
when `include` is set cargo does NOT consult .gitignore. Packaging from a dirty
working tree therefore swept 98 junk files (`.claude/worktrees/agent-*/...`
READMEs) into the local v0.6.0 crate because patterns like `README.md` and
`docs/design-system/copy.toml` match at any depth. CI publishes from a clean
checkout so shipped crates are unaffected, but a local `cargo publish` would
leak.

## Task

Anchor every include pattern with a leading `/` (e.g. `/README.md`,
`/docs/design-system/copy.toml`, `/src/**/*.rs`) and add a packaging test or
CI check asserting `cargo package --list` contains no path outside the
expected roots.
