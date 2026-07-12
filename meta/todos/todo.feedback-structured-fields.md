---
node: cairn.kernel.cli
status: open
created: 2026-07-12
---

# Feedback Structured Fields

gh:#246

`cairn feedback` lacks --area/--severity structured fields and truncates titles.

## Evidence (verified on main, 2026-07-12)
- `src/cli/commands/feedback.rs:11` joins all args into the message; flags are
  swallowed into the title.
- `src/cli/commands/feedback.rs:55-58` truncates the generated title at 80
  chars mid-word (reproduced in scratch probe).

## Task
Parse `--area` and `--severity` flags, include them in the generated issue body
and any `--json` output, and truncate titles at a word boundary (or stop
truncating and let GitHub handle length).
