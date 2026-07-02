# Spec: Phase 7 — Change-system trim

## Acceptance criteria

- `grep -rn "create_change_epic\|claim_change\|storage_backend" src/` returns
  no hits.
- `cairn change new demo && cairn changes` still scaffolds and lists the
  change (format intact); the demo directory is removed after verification.
- Full `cargo test` is green after deletion.
- `crate::state::backlog::` callsites are unchanged; the read-only backlog
  view still works.
