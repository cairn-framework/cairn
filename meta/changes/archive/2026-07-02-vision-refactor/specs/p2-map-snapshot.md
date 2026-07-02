# Spec: Phase 2 — Persistent map snapshot

## Acceptance criteria

- Running `scan` twice on an unchanged tree produces a byte-identical
  `map.json` (`git diff --quiet map.json` after the second run exits 0).
- `map.json`'s `schema_version` is `1`.
- Mutating one source file's symbol and re-scanning changes only that node's
  `symbols` entry in `map.json`; nothing else in the snapshot changes.
- `map.json` is committed at this repo's root (not gitignored).
