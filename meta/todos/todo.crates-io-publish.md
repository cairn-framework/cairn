---
node: cairn.root
status: open
created: 2026-07-03
---

# Crates Io Publish

Publish cairn to crates.io as `cairn-framework` (see `dec.native-todos-first`
companion work and `dec.beads-task-layer`); binary names stay
`cairn`/`cairn-mcp`/`cairn-lsp`. `cairn-macros` publishes first since
`cairn-framework` depends on it by path. Superseded framing: the original
bead considered `cairn-cli` as the alt name and deferred publishing
entirely; the crates.io publish is no longer deferred.

**Status as of v0.1.1 (2026-07-03):** rename to `cairn-framework` shipped
(Cargo.toml, README, quickstart, CHANGELOG); `cargo publish --dry-run
--allow-dirty -p cairn-macros` and `-p cairn-framework` both verified clean
against the live crates.io index. The actual publish is blocked on a
crates.io API token: browser automation against the token-creation page
failed (SPA click/fill timeouts across multiple selector strategies), and
no token was pasted when asked. Publish order stays `cairn-macros` (0.1.0)
then `cairn-framework` (0.1.1), each with a dry-run first (publishes are
yank-only, not reversible). v0.1.1 ships without a crates.io presence;
prebuilt binaries (curl/PowerShell installer) and the Homebrew tap
(`cairn-framework/homebrew-tap`) are the supported install paths meanwhile.

bd:cairn-m99
