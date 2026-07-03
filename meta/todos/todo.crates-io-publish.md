---
node: cairn.root
status: blocked
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
no token was pasted when asked. Acquire one manually at
https://crates.io/settings/tokens with `publish-new` + `publish-update`
scopes, then `cargo login <token>`. Publish order stays `cairn-macros`
(0.1.0) then `cairn-framework` (0.1.1), each with a dry-run first
(publishes are yank-only, not reversible); drop `--allow-dirty` for the
real publish, it was only needed for this session's uncommitted-tree
dry-runs. v0.1.1 ships without a crates.io presence; prebuilt binaries
(curl/PowerShell installer) and the Homebrew tap
(`cairn-framework/homebrew-tap`) are the supported install paths meanwhile.

**Automation added (2026-07-03):** publishing is now wired into the
release pipeline as a custom dist publish-job
(`.github/workflows/cargo-publish.yml`, `publish-jobs = ["homebrew",
"./cargo-publish"]` in `dist-workspace.toml`). It runs automatically on
every future tagged release, right where `publish-homebrew-formula`
runs, publishing `cairn-macros` then `cairn-framework` in dependency
order. It is idempotent (checks the crates.io API per crate/version and
skips an already-published one, so `cairn-macros` staying at 0.1.0
across future `cairn-framework` releases will not hard-fail) and skips
prereleases like the Homebrew job does. Peer-reviewed clean (idempotency,
publish order, prerelease gating, secret handling, permissions scope all
verified against the live crates.io API and the pre-existing
`legacy-installer-continuity.yml`/`publish-homebrew-formula` patterns).
`CARGO_REGISTRY_TOKEN` was added to the repo secrets on 2026-07-03
(verified via `gh secret list`). v0.1.1 itself was never retroactively
published (that release already shipped without CI's cargo-publish
step).

**v0.1.2 hit a real bug (2026-07-03):** its release run was the first
to exercise the job and failed: crates.io's API enforces a data-access
policy that 403s a generic User-Agent (a bare `curl/<version>` counts
as one); the `already_published()` existence-check curl call didn't
set one, so the release failed before either crate published (the
GitHub release, binaries, and Homebrew formula all still shipped fine
for v0.1.2, since that job runs independently). Fixed in
`cargo-publish.yml` by adding `-A "$UA"` to both curl call sites.
Because a local relative-path `workflow_call` ref resolves at the
caller's commit SHA, the fix could not be applied by re-running the
v0.1.2 tag's workflow run; v0.1.3 was cut to retry cleanly. This stays
open/blocked until v0.1.3's release run confirms both crates land on
crates.io.

bd:cairn-m99
