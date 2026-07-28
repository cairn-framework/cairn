# Tasks: Contract node-shape drift

This change is the proposal; tasks 1 to 5 land with it. Tasks 6 and 7 run in
that order: the re-record surface is the prerequisite that unblocks the
enforcer. Their canonical trackers are
`meta/todos/todo.contract-baseline-rerecord-surface.md` and
`meta/todos/todo.contract-blueprint-staleness.md`.

- [x] Settle tier, finding code, baseline schema and migration, and recording
      point in `design.md`, with the rejected alternatives argued
- [x] Write the enforcer's acceptance criteria to `specs/`
- [x] Add the `pending` rule row to `docs/registries/spec-rules.md`, leaving its
      `Code` cell empty: `docs/conventions.md` rule 2 binds the registry number
      to the commit that introduces the code in Rust
- [x] Add the finding's user-facing text to `docs/design-system/copy.toml`
- [x] Author the re-record prerequisite the summariser-disabled evidence proved
      necessary, and list it in the enforcer todo's `Depends on`
- [ ] Build the non-generative baseline re-record surface, with record and drop
- [ ] Implement the enforcer against `specs/contract-node-shape-drift.md`,
      allocate its code in `docs/registries/error-codes.md`, fill the rule row's
      `Code` cell, and promote the row from `pending` to `enforced`
