---
node: cairn.kernel.cli
status: blocked
created: 2026-07-13
related: [dec.loop-command-harness-model]
---

# Land the /cairn-loop harness-mode rewrite

The rewritten command (~246 lines, `.claude/commands/cairn-loop.md`) sits
uncommitted in worktree `../cairn-loop-rewrite`, branch `omp-loop-cairn-loop`.
Design reviewed this session: adversarial reviewer pass (3 blockers fixed),
contradiction audit, and ablation audit all folded in.

## Blocked on

dec.loop-command-harness-model was ratified 2026-07-13, so the design gate is
cleared. Still blocked on ONE thing: the owner's explicit authorization to
land (commit, push, PR). Do not land before that word.

## Landing sequence (when unblocked)

Commit on the branch, push, one PR, two independent read-only reviews of the
diff (correctness lens plus simplicity lens), CI green, squash-merge, prune
branch and worktree. The same scoped change executes the workflow-doc
retirement (decision point 8): migrate the live consumers of
`docs/agent/cairn-dev-workflow.md` (CLAUDE.md reference, session handoff,
todo pointers), extract the recovery and landing procedure skills, and leave
a short descriptive overview in place of the doc.
