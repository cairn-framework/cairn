# Refactoring

Goal: change structure without changing behaviour, and prove behaviour did not
change.

## Query sequence

```bash
cairn get <node> --json                  # what the node owns today
cairn deps <node> --direction in --transitive   # everyone who would feel a move
cairn deps <node> --transitive           # everything this node leans on
cairn rationale <node>                   # decisions that fixed the current shape
cairn contract <node>                    # the interface you must preserve
cairn neighbourhood <node> --include-todos --include-changes
```

The inbound transitive set is the one that matters. It is the list of nodes whose
tests must still pass, and the reason a refactor is riskier than its diff looks.

`cairn rationale` is not optional here. A shape that looks accidental is often
load-bearing, and an accepted decision explaining it outranks your judgement. If
you intend to contradict one, write a superseding decision; do not just refactor
past it.

## When the graph stops helping

- Renaming or moving a symbol across files is a language-server job. Use its
  rename refactor so every call site moves; text substitution silently misses
  re-exports and shadowed names.
- Cairn edges are module-level. To find the specific call sites inside a node, use
  the language server's references, not `cairn deps`.
- Read spans, not files: `cairn get <node> --symbols --json` gives `line` and
  `end_line` per symbol.

## Moving files between nodes

A refactor that moves files changes ownership, so the blueprint moves with it:

1. Update the node `path` entries so every moved file is owned again.
2. Add or remove edges if the call graph between modules changed.
3. `cairn scan` and confirm zero `CAIRN_RECONCILE_ORPHANED_FILE`.
4. Expect `CAIRN_INTERFACE_HASH_CHANGED` if a public interface moved. That finding
   is the point: confirm the change is intended before clearing it.

## Verify

Behaviour preservation is the claim, so the evidence is the existing suite
passing unchanged, plus:

```bash
cairn scan
cairn hook all
```

Do not add new behaviour in a refactor. If you find a bug while refactoring, land
the refactor first and fix the bug separately, so the diff that changes behaviour
is reviewable on its own.
