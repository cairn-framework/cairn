---
node: cairn.kernel.cli
status: done
created: 2026-07-25
---

# Pack Campaign Resolver

Third child of `todo.agent-pack-claude-bootstrap`. Makes an installed pack
resolvable as an immutable set of bytes for the duration of a campaign.

## Priority

P2. Integrity work that only means anything once a pack is installed and wired.

## Depends on

`todo.pack-install-lifecycle`, `todo.pack-init-delegation`.

## Scope

- Render Claude-native invocations from the canonical `cairn-dev` entry and its
  modes. A native loop command is transport for loop mode, not a second
  authority (`dec.unified-cairn-dev-entry` clause 8).
- Install an adapter-owned script or native helper that deterministically
  resolves the selected local prompt bytes plus bundle, CLI, and required-asset
  hashes. It never selects work, starts sessions, retries, or interprets
  terminal tokens (`dec.no-orchestrator`).
- Publish the installed ownership manifest last through an atomic replacement.
  At campaign start the resolver reads its generation, buffers the complete
  ordered closure, hashes those exact bytes, and rereads the generation. If
  both reads and all hashes agree it writes an immutable campaign snapshot;
  prompt execution and procedure loading use only that snapshot. Concurrent,
  interrupted, mixed-revision, or post-check mutation fails before work
  (`dec.unified-cairn-dev-entry` clause 9).
- "Latest" means the locally installed pack selected at invocation. Never fetch
  or update the pack while resolving a prompt.

## Acceptance

- A smoke scenario proves that the installed Claude-native loop invocation
  resolves canonical `cairn-dev` loop mode and its currently declared required
  asset closure, which now includes `cairn-loop-reconcile`
  (`dec.loop-reconcile-step`).
- A campaign scenario records that closure in external harness state, verifies
  the same bytes before each fresh session, and halts before work on mismatch.
- Ordinary pack drift stays info only outside an active campaign.
- Jointly with the foundation and its sibling children, this satisfies the
  Claude-applicable clauses of `dec.agent-pack-packaging`.

