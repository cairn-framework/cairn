---
node: cairn.kernel.cli
status: open
created: 2026-07-22
---

# Agent Pack Claude Bootstrap

## Priority

P1. The first end-to-end adoption slice.

## Depends on

Lifecycle implementation depends on `todo.agent-pack-canonical-foundation`.
Canonical entry-mode migration and loop campaign smoke also depend on
`todo.agent-guidance-router-playbooks` and its refining decision.

## Scope

Implement the Claude path of the ratified pack lifecycle before evaluating
adoption:

- One `install | update | status | uninstall` command family with the Claude
  harness selector and auto-detection.
- An installed ownership manifest carrying schema version, installed CLI
  version, bundle version, and per-file content hashes.
- Matching-hash-only updates and removals. Preserve hand-authored skills and
  AGENTS/CLAUDE content.
- Info-only version drift, missing-file backfill, retire-if-pristine,
  legacy-install adoption, and compiled migration notes.
- Resolved symlink containment and destination-alias rejection before any
  write, using the foundation's shared preflight.
- `cairn init --wire` delegates to the installer.
- After the refining decision and router change, render Claude-native
  invocations from the canonical `cairn-dev` entry and modes. A native loop
  command is transport for `cairn-dev` loop mode, not a second authority.
- Install an adapter-owned script or native helper that deterministically
  resolves the selected local prompt bytes plus bundle, CLI, and
  required-asset hashes. It never selects work, starts sessions, retries, or
  interprets terminal tokens.
- Publish the installed ownership manifest last through an atomic replacement.
  At campaign start, the resolver reads its generation, buffers the complete
  ordered closure, hashes those exact bytes, and rereads the generation. If
  both reads and all hashes agree, it writes an immutable campaign snapshot;
  prompt execution and procedure loading use only that snapshot. Concurrent,
  interrupted, mixed-revision, or post-check mutation fails before work.
- Define "latest" as the locally installed pack selected at invocation.
  Never fetch or update the pack while resolving a prompt.

## Close the first-run gap

- Make `cairn init --wire` the documented greenfield path.
- Support `cairn init --from-code --apply --wire [path]`.
- After successful brownfield apply, backfill `.cairn/AGENTS.md` and the
  rendered pack, then wire the selected agent instructions file.
- Never scaffold or wire after a failed brownfield apply.

## Acceptance

- Fresh greenfield and brownfield repositories both reach an installed and
  wired Claude pack with one command.
- Re-running each path is idempotent.
- Modified user files are reported and never overwritten.
- A smoke scenario proves that the installed Claude-native loop invocation
  resolves canonical `cairn-dev` loop mode and its currently declared required
  asset closure.
- A campaign scenario records that closure in external harness state, verifies
  the same bytes before each fresh session, and halts before work on mismatch.
  Ordinary pack drift remains info-only outside an active campaign.
- README, quickstart, agent setup, command help, copy, and snapshots describe
  the behaviour that actually shipped.
- Jointly with the foundation, this satisfies the Claude-applicable clauses of
  `dec.agent-pack-packaging`.

