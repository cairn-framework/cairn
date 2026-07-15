---
node: cairn.kernel.cli
status: done
created: 2026-07-13
related: [dec.cairn-brief-orientation]
---

# Land the brief generic-gates fix

Branch `loop/brief-generic-gates` (worktree `../cairn-brief-gates`), commit
0993c22, unpushed. Scoped tests pass; no rebase was needed when checked
2026-07-13.

## Problem it fixes

Owner field report (2026-07-13): a downstream TypeScript repo received
cargo-specific gate text (`cargo build`, `cargo clippy`) and `bd ready`
staleness guidance from `cairn brief` output, because the `[brief]` table in
`docs/design-system/copy.toml` hardcodes cargo gates and the staleness note
references beads regardless of the target repo's actual configuration.

## Fix in the commit

Gates become repo-aware (derived from the target repo's hook config and
AGENTS.md, never cairn's own), and the staleness note is conditional on the
actual BriefSource (beads vs native todos). Diff limited to copy.toml,
src/cli/render/remediate.rs, and tests.

## Remaining

Merged as PR #315 (squash commit b94620b on main); an independent review approved it with no blocking findings.
