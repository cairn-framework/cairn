# Design: Phase 4 Hooks

## References

- `docs/spec.md` section 11 for hook classes.
- `docs/spec.md` sections 9 and 16 for active change conflict timing.

## Hook Engine

The hook engine SHALL run scan or lint once, classify findings, and return `HookReport`.

```rust
pub enum HookKind {
    Structural,
    Interface,
    Tension,
    All,
}
```

`HookReport` SHALL include hook kind, findings, conflict findings, exit decision, elapsed time, and output paths touched.

## Exit Semantics

- Structural hook exits `1` when structural errors exist.
- Interface hook exits `1` when unresolved interface contradictions exist.
- Tension hook always exits `0` after reporting tensions.
- All hook exits `1` if structural or interface hook fails; tensions alone do not fail it.

## Conflict Detection

The engine SHALL compare active change directories and detect:

- Multiple changes modifying the same DSL node or edge.
- A change removing a node that another change modifies.
- Multiple changes modifying the same artefact path.
- Rename chains that target the same old or new ID.

Conflicts SHALL be structural hook failures because archive safety is compromised.

## Entrypoints

The CLI SHALL expose:

- `cairn hook structural`
- `cairn hook interface`
- `cairn hook tension`
- `cairn hook all`

All hook commands SHALL support `--json`, `--file`, and `--changes-dir`.

Repository scripts SHALL include a committed hook runner suitable for Git pre-commit and agent-task-end use. The script SHALL invoke `cairn hook all`.

## Testing

Tests SHALL cover exit codes, finding classification, JSON output, human output, conflict classes, script invocation, and non-blocking tension behavior.
