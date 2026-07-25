# Feature implementation

Goal: add behaviour that the graph still describes correctly afterwards.

## Query sequence

```bash
cairn context                            # find the area that should own this
cairn get <node> --json                  # confirm ownership and current paths
cairn contract <node>                    # the interface you are extending
cairn rationale <node>                   # decisions that constrain the design
cairn deps <node> --direction in         # who consumes what you are about to change
cairn todos <node> --status open         # work already claimed here
cairn neighbourhood <node> --include-changes    # an active change may already touch it
cairn frontier                           # if the feature is a declared ghost node
```

Settle ownership before writing code. A feature implemented in the wrong node
produces a correct diff and a wrong graph, and the fix is a later migration.

## Before you write

Name the outcome, the nearest place it becomes observable, and the evidence that
will prove it there. For substantial work, capture that in a change first with the
`cairn-propose` skill, which exists for exactly this.

## While you write

- Every new file falls under a node `path`, tests included. If none fits, extend a
  node's paths or declare a new Module.
- Every new cross-module call gets a blueprint edge:
  `from.id -> to.id "relationship label"`. Check for a cycle first with
  `cairn deps <target> --transitive`.
- User-facing strings belong wherever the repository centralises them; check its
  conventions rather than hardcoding.
- Changed behaviour gets a test.

## When the graph stops helping

The graph tells you where the feature belongs and what it may touch. It does not
design the feature. Read the contract and the neighbouring implementation to match
existing patterns, using `--symbols` spans rather than whole files, and use the
language server to check that a new signature does not break an existing caller.

## Extending a ghost node

`cairn frontier` lists declared-but-unbuilt nodes that are buildable now, meaning
their dependencies exist. Implementing one is the cheapest kind of feature: the
node, its edges, and often its contract are already declared, so the work is to
make the code match the declaration and let `cairn scan` confirm the transition
from ghost to synced.

## Verify

```bash
cairn scan
cairn hook all
```

Plus the repository's own gates, plus evidence at the boundary the feature is
claimed at. If structure changed or you made a non-obvious tradeoff, record a
decision in `meta/decisions/` (`cairn decision new <slug> --node <id>`).
