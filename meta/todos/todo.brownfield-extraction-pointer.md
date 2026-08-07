---
node: cairn.brownfield
status: blocked
created: 2026-08-07
blocked_by: [todo.brownfield-extraction-flow]
parent: todo.brownfield-decision-extraction
---

# Point post-install output at the decision-extraction flow

Wiring unit split out of `todo.brownfield-decision-extraction` under the sizing
rule. Blocked until `todo.brownfield-extraction-flow` exists, because a pointer
to a flow that does not exist is worse than no pointer.

## Problem

The post-install dead end is recorded in
`src.reddit-gregerw-first-user-test`. The existing `[init]` copy blocks in
`docs/design-system/copy.toml` (keys `next-steps`, `next-steps-wired`,
`next-steps-existing`, `next-steps-existing-wired`) stop at scan, status, and
wiring: none names a step that produces visible value on the user's own code.

## Task

Name the extraction flow as a next step in the post-install surfaces:
`cairn init` (including `--from-code`) and `cairn onboard` output. Every string
goes in `docs/design-system/copy.toml`; nothing is hardcoded in Rust source.

Keep the brownfield and greenfield paths distinct: the extraction step belongs
on the `--from-code` and existing-blueprint variants, where there is code to
mine, not on the empty-project variant.

## Acceptance

- Running `cairn init --from-code` on a brownfield fixture prints a next step
  naming the extraction flow, and `cairn onboard` output does the same.
- The added strings live in `docs/design-system/copy.toml` and are asserted by a
  test on the rendered output, not on the copy table.
- `cairn scan --strict` exits 0.
