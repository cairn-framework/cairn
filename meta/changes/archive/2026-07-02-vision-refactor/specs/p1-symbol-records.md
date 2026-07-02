# Spec: Phase 1 — Symbol records

## Acceptance criteria

- `cairn symbols <synced-node> --json` returns a non-empty `symbols` array
  whose entries all carry `name`, `kind`, `signature`, `file`, `line`.
- `cargo test --test scanner_interface_hash` passes unchanged: fingerprint
  hashes did not move when `SymbolRecord` extraction was added.
- `cairn symbols` on an unknown node returns the same `Finding` error shape as
  `cairn files`.
- The webui module inspector renders a "Symbols" section for a node with
  extracted symbols, and a defined empty state for a node with none.
