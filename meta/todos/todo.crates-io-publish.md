---
node: cairn.root
status: done
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

**v0.1.3 hit a second, unrelated issue (2026-07-03):** the UA fix
worked (no 403), and `cairn-macros` 0.1.0 published successfully
(confirmed live via the crates.io API), but the `cairn-framework`
0.1.3 upload then failed 3 times in a row with a 503 "backend write
error" from crates.io's Varnish layer, not flagged on
status.crates.io. This looks like a real, if unlisted, outage on
crates.io's write path specifically (reads are unaffected).
`cargo-publish.yml` was hardened in `a5fc3ef` to retry the actual
`cargo publish` call up to 3x with backoff, re-checking
`already_published()` at the top of each iteration so a publish that
lands server-side despite a client-side error is detected and skipped
rather than double-attempted; this only takes effect on the *next*
tagged release (`workflow_call` resolves at the caller's commit SHA,
same constraint as the UA fix).

**Resolved in v0.1.4 (2026-07-03):** the 503s turned out not to be a
pure transient outage after all. The 4th retry surfaced the real
root cause: a 413 Payload Too Large. The default package (no
`include`/`exclude` in Cargo.toml) shipped the entire repo (demo
GIFs/MP4s, a PDF, .beads bookkeeping, archived research
screenshots, test fixtures): 1147 files, 42.7MiB compressed, well
over crates.io's 10MB upload cap. Fixed in `ee176f1` by adding an
`include` allowlist scoped to what `src/**` needs to compile plus
its `include_str!`-embedded runtime assets (webui HTML/JS/CSS, the
agent guide, the 8 bundled skill files). Verified via a real
`cargo package` build (not --no-verify): 286 files, 491.9KiB
compressed, compiles standalone. v0.1.4's release run published
both crates successfully: `cairn-macros` 0.1.0 (already live,
skipped) and `cairn-framework` 0.1.4 (new). Both confirmed live on
crates.io via the API (200). `cargo install cairn-framework
--version 0.1.4` verified in a scratch CARGO_HOME.
bd:cairn-m99
