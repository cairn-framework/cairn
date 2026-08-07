---
node: cairn.root
status: open
created: 2026-08-07
related: [res.parallel-dispatch-rung-3]
---

# Root anchor hygiene: re-anchor the catch-all todos

Measured 2026-08-07 (`res.parallel-dispatch-rung-3`, "What derived-first
actually buys"): 16 of 39 open todos anchor to `cairn.root`, a Module owning
seven specific entry-point files, yet much of that work is repository-wide or
process work touching none of them. Under rung 3's derived write-sets those 16
units can never co-dispatch (at most one per wave), and their derived write-set
is simultaneously too coarse and too narrow.

## Task

Walk the open `cairn.root`-anchored todos and re-anchor each to the node whose
files it actually touches, or record one line of justification where
`cairn.root` is genuinely right. Use `cairn todo set` / frontmatter edits via
the sanctioned verbs only. No schema change; this is graph hygiene, and it
raises achievable wave width without any new authoring contract.

## Acceptance

- Every open todo anchored to `cairn.root` either moved to a truer node or
  carries a stated justification.
- `cairn scan --strict` stays green.
