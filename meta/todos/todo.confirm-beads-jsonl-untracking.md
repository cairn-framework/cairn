---
node: cairn.state
status: open
created: 2026-07-11
---

# Confirm Beads Jsonl Untracking

Ratify or veto the resolution of
`dec.gap-cairn-state-should-beads-jsonl-exports-stay-git-tracked-now`:
`.beads/*.jsonl` exports were untracked (gitignored + removed from index) in
an unattended run after the ask timed out and auto-accepted the recommended
option.

To ratify: flip this todo's `status` to `done` when approving/merging the PR
that carries this change. To veto: revert the ignore rule, `git add` the
exports back, flip the gap decision back to `proposed`, and mark this todo
`done` with a note.
