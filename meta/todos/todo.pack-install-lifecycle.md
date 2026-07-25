---
node: cairn.kernel.cli
status: open
created: 2026-07-25
---

# Pack Install Lifecycle

First child of `todo.agent-pack-claude-bootstrap`. Owns the install verb and
the ownership ledger; the other two children build on this one.

## Priority

P1. Nothing else in the Claude bootstrap can land without an installer and a
manifest to record what it owns.

## Depends on

`todo.agent-pack-canonical-foundation` (done; `tools/agent-pack` renders the
canonical content into the `.claude` destinations the binary compiles in).

## Scope

- One `cairn pack install | update | status | uninstall` command family with a
  `--harness <name>` selector and auto-detection, per
  `dec.agent-pack-packaging` clause 4. No per-harness command fan-out.
- An installed ownership manifest in the target repo recording schema version,
  installed CLI version, bundle version, and per-file content hashes
  (clause 5). It is the only record of what the packager owns.
- Write, refresh, or retire ONLY manifest-listed files at a matching hash.
  Modified files are reported and never overwritten or deleted; hand-authored
  skills and existing AGENTS or CLAUDE content are never touched (clauses 5
  and 8).
- Three-way per-file handling on update: pristine refreshes, modified reports,
  missing backfills. Removal is retire-if-pristine (clause 6).
- Version drift is info level, never blocking, and names the update verb
  (clause 6).
- A skills directory with no manifest is a legacy install: scan it, match
  against the bundled pack, write the manifest rather than clobbering.
- Migration notes ship compiled into the binary keyed by version range; no
  fetch at any point.
- Resolved symlink containment and destination-alias rejection before any
  write, reusing the foundation's shared preflight.
- Loop mode stays opt in: the base install must not install `loop-mode.md` or
  its procedure closure unless explicitly requested, because the shipped
  `cairn-dev` router treats their absence as "loop mode is unavailable here".

## Acceptance

- Fresh install into an empty repository produces the pack plus a manifest, and
  re-running is idempotent (no rewrite, no duplicate manifest rows).
- A user-modified pack file is reported by `status` and survives `update` and
  `uninstall` untouched.
- A pack directory with no manifest is adopted, not overwritten.
- Symlinked or aliased destinations are rejected before any byte is written.
- Command help, `docs/commands.md`, `docs/integration-contract.md`, the
  consistency test list, and `docs/design-system/copy.toml` describe what
  shipped.

