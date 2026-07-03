---
node: cairn.root
status: done
created: 2026-07-03
---

# Homebrew Tap

Deferred at v0.1.0 launch: cargo-dist can add a Homebrew formula/tap later
without rework.

Resolved: created `cairn-framework/homebrew-tap` (initialized with a
README so `main` exists as a real branch cargo-dist can push to). Added
`"homebrew"` to `dist-workspace.toml`'s `installers` (alongside the
existing `shell`/`powershell`), `tap = "cairn-framework/homebrew-tap"`,
and `publish-jobs = ["homebrew"]`. Added `[package.metadata.dist]
formula = "cairn"` to Cargo.toml so the formula name matches the CLI
binary (`cairn-framework` is the crates.io package name, `cairn` is the
installed command). The resulting user command is
`brew install cairn-framework/tap/cairn`. Set the `HOMEBREW_TAP_TOKEN`
repo secret using the already-authenticated `gh` CLI token (scope
`repo`, sufficient for cross-repo write to the tap). Verified via
`dist plan`: matrix now includes `cairn.rb` alongside the existing
shell/powershell installers and all five build targets. The formula
goes live on the tap with the v0.1.1 tag push (Phase 7).

bd:cairn-jj4
