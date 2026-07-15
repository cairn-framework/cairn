---
node: cairn.kernel.cli
status: open
created: 2026-07-13
related: [dec.agent-pack-packaging]
---

# Implement the agent pack per dec.agent-pack-packaging

dec.agent-pack-packaging was ratified 2026-07-13, so this is now buildable.
Scaffold a change directory (`cairn change new`) rather than implementing
piecemeal; keep to the ratified contract and the scope below.

## Scope when unblocked (from the decision's contract)

- Canonical harness-neutral pack source at repo root with per-harness adapter
  rows as data; this repo's `.claude/skills/` cairn-* pack becomes rendered
  output with a byte-for-byte drift gate (extending wire_format_snapshots
  discipline).
- Build-time renderer (dev-only), not shipped in the user CLI.
- Pack lifecycle verbs `install | update | status | uninstall` with
  `--harness` selector and auto-detection; `init --wire` delegates.
- Ownership manifest in target repos (schema version, CLI version, bundle
  version, per-file hashes); three-way drift handling, retire-if-pristine,
  legacy-adopt; compiled-in migration notes.
- First harness targets: Claude Code (verified), OMP second. Other adapters
  only with live-harness validation.
