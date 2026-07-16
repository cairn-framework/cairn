---
node: cairn.kernel.cli
status: done
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

## Resolution (2026-07-16)

Shipped in `src/cli/commands/feedback.rs`: `--area` and `--severity` are now
parsed out of the args (with a "requires a value" error on a missing or
flag-shaped value) instead of being joined into the message. The values land
in the feedback log entry, the prefilled issue body, and the `--json` data
object as optional `area` / `severity` fields. Generated titles still cap at
80 characters but now cut at the last word boundary, falling back to a hard
cut only when the prefix has no whitespace; the cap was kept because GitHub
accepts long titles but renders them poorly in lists. Per-command help
(`src/cli/help/mod.rs`, copy keys in `docs/design-system/copy.toml`) and
`docs/commands.md` advertise the real flags. Tests cover flag parsing,
body/JSON inclusion, and word-boundary truncation.
